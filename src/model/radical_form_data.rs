use std::{sync::Arc, collections::VecDeque, ops::Index};

use daggy::{NodeIndex, Walker};
use serde::{Deserialize, Serialize};

use crate::model::radical_from_csv;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RadicalFormData {
    pub radical_form: char,
    pub radical_number: i32,
    pub current_understanding: UnderstandingLevel,
}

impl QuestionAnswerPrompt for RadicalFormData {
    fn current_understanding(&self) -> UnderstandingLevel {
        self.current_understanding.clone()
    }

    fn question_prompt(&self) -> String {
        todo!()
    }

    fn process_answer_input(&self, answer: String) -> bool {
        todo!()
    }
}

pub trait QuestionAnswerPrompt {
    fn current_understanding(&self) -> UnderstandingLevel;
    fn question_prompt(&self) -> String;
    fn process_answer_input(&self, answer: String) -> bool;
}

pub struct LearningSession {
    overall_learning_frame: LearningFrame,
    questions_dag: QuestionsDAG,
}

impl LearningSession {
    pub fn start_session(&mut self) {
        self.single_run_through_frame();
    }

    pub fn single_run_through_frame(&mut self) {
        for prompt_i in self.overall_learning_frame.prompts.iter() {
            let node = &self.questions_dag[*prompt_i];
            node.qap.current_understanding();
        }
    }
}

#[derive(Clone)]
pub struct LearningFrame {
    pub size: usize,
    pub prompts: Vec<NodeIndex>
}

impl LearningFrame {
    
    pub fn merge(mut self, mut other: LearningFrame) -> Self {
        self.size = self.size + other.size;
        self.prompts.append(&mut other.prompts);
        self
    }

    pub fn split(mut self) -> (LearningFrame, Option<LearningFrame>) {
        if self.size < 6 {
            (self, None)
        } else {
            let half_size = self.size / 2;
            let other = LearningFrame {
                size: self.size - half_size,
                prompts: self.prompts.split_off(half_size),
            };
            self.size = half_size;
            (self, Some(other))
        }
    }

    pub fn split_cloned(&self) -> (LearningFrame, Option<LearningFrame>) {
        let mut cloned = self.clone();
        let (first, other) = cloned.split();
        (first, other)
    }

}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum UnderstandingLevel {
    DontKnow,
    Know,
    InstantRecall(i32, i32) // excluded_from_frame_of_size, correct answer streak
}

pub fn start_learning_session() {

}

pub struct QANode {
    qap: Arc<dyn QuestionAnswerPrompt>,
}

impl QANode {
    pub fn new(qap: Arc<dyn QuestionAnswerPrompt>) -> Self {
        Self {qap}
    }
}

pub struct QuestionsDAG {
    dag: daggy::Dag<QANode, ()>,
}

impl QuestionsDAG {
    pub fn find_shallow_node(&self, start: NodeIndex) -> Option<NodeIndex> {
        let mut queue = VecDeque::new();
        let mut visited = vec![false; self.dag.node_count()];
        let mut know_node = None;

        queue.push_back(start);

        while let Some(node_index) = queue.pop_front() {
            if visited[node_index.index()] {
                continue;
            }
            visited[node_index.index()] = true;

            let node = &self.dag[node_index];
            match node.qap.current_understanding() {
                UnderstandingLevel::DontKnow => return Some(node_index),
                UnderstandingLevel::Know => {
                    if know_node.is_none() {
                        know_node = Some(node_index);
                    }
                },
                _ => (),
            }

            let walker = self.dag.children(node_index);
            for (_edge_index, child_node_index) in walker.iter(&self.dag) {
                queue.push_back(child_node_index);
            }
        }

        know_node
    }
}

impl Index<NodeIndex> for QuestionsDAG {
    type Output = QANode;
    fn index(&self, index: NodeIndex) -> &QANode {
        &self.dag[index]
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::{sync::Arc, collections::HashMap};
    use daggy::NodeIndex;
    use paste::paste;

    macro_rules! create_nodes {
        ($dag:ident, $indices:ident, $( $var:ident : $level:expr ),* ) => {{
            $(
                let $var = Arc::new(MockQAP { understanding: $level });
                paste! {
                    let [<$var _node>] = QANode::new($var);
                    let [<$var _index>] = $dag.add_node([<$var _node>]);
                    $indices.insert(stringify!($var), [<$var _index>]);
                }
            )*
        }};
    }

    macro_rules! ind {
        ($dag:ident, $indices:ident, $( $var:ident : $level:expr ),* ) => {{
            $(
                let $var = Arc::new(MockQAP { understanding: $level });
                paste! {
                    let [<$var _node>] = QANode::new($var);
                    let [<$var _index>] = $dag.add_node([<$var _node>]);
                    $indices.insert(stringify!($var), [<$var _index>]);
                }
            )*
        }};
    }

    struct MockQAP {
        understanding: UnderstandingLevel,
    }

    impl QuestionAnswerPrompt for MockQAP {
        fn current_understanding(&self) -> UnderstandingLevel {
            self.understanding.clone()
        }
    }

    #[test]
    fn test_find_shallow_node() {
        let qap1 = Arc::new(MockQAP { understanding: UnderstandingLevel::Know });
        let qap2 = Arc::new(MockQAP { understanding: UnderstandingLevel::DontKnow });
        let qap3 = Arc::new(MockQAP { understanding: UnderstandingLevel::InstantRecall(1, 2) });

        let node1 = QANode::new(qap1);
        let node2 = QANode::new(qap2);
        let node3 = QANode::new(qap3);

        let mut dag = daggy::Dag::<QANode, ()>::new();
        let index1 = dag.add_node(node1);
        let index2 = dag.add_node(node2);
        let index3 = dag.add_node(node3);

        dag.add_edge(index1, index2, ()).unwrap();
        dag.add_edge(index1, index3, ()).unwrap();

        let questions = QuestionsDAG { dag };

        assert_eq!(questions.find_shallow_node(index1), Some(index2));
    }

    #[test]
    fn test_find_shallow_node_no_dontknow_or_know() {
        let qap1 = Arc::new(MockQAP { understanding: UnderstandingLevel::InstantRecall(1, 2) });
        let qap2 = Arc::new(MockQAP { understanding: UnderstandingLevel::InstantRecall(2, 3) });

        let node1 = QANode::new(qap1);
        let node2 = QANode::new(qap2);

        let mut dag = daggy::Dag::<QANode, ()>::new();
        let index1 = dag.add_node(node1);
        let index2 = dag.add_node(node2);

        dag.add_edge(index1, index2, ()).unwrap();

        let questions = QuestionsDAG { dag };

        assert_eq!(questions.find_shallow_node(index1), None);
    }

    #[test]
    fn test_deep_dontknow_end_of_chain() {
        let mut dag = daggy::Dag::<QANode, ()>::new();
        let mut indices = HashMap::new();
        create_nodes!(dag, indices,
            qapd: UnderstandingLevel::DontKnow, 
            qapk1: UnderstandingLevel::Know, 
            qapk2: UnderstandingLevel::Know, 
            qapk3: UnderstandingLevel::Know, 
            qapk4: UnderstandingLevel::Know, 
            qapi1: UnderstandingLevel::InstantRecall(2, 3), 
            qapi2: UnderstandingLevel::InstantRecall(2, 3)
        );

        add_edge(&mut dag, &indices, "qapi1", "qapi2");
        add_edge(&mut dag, &indices, "qapi2", "qapk1");
        add_edge(&mut dag, &indices, "qapk1", "qapk2");
        add_edge(&mut dag, &indices, "qapk2", "qapk3");
        add_edge(&mut dag, &indices, "qapk3", "qapk4");
        add_edge(&mut dag, &indices, "qapk4", "qapd");
        let questions = QuestionsDAG { dag };

        let qapi1_index = indices.get("qapi1").unwrap();
        let qapd_index = indices.get("qapd").unwrap();

        assert_eq!(questions.find_shallow_node(*qapi1_index), Some(*qapd_index));

    }

    #[test]
    fn test_deep_chain_only_at_most_know() {
        let mut dag = daggy::Dag::<QANode, ()>::new();
        let mut indices = HashMap::new();
        create_nodes!(dag, indices,
            qapkend: UnderstandingLevel::Know, 
            qapk1: UnderstandingLevel::Know, 
            qapk2: UnderstandingLevel::Know, 
            qapk3: UnderstandingLevel::Know, 
            qapk4: UnderstandingLevel::Know, 
            qapi1: UnderstandingLevel::InstantRecall(2, 3), 
            qapi2: UnderstandingLevel::InstantRecall(2, 3)
        );

        add_edge(&mut dag, &indices, "qapi1", "qapi2");
        add_edge(&mut dag, &indices, "qapi2", "qapk1");
        add_edge(&mut dag, &indices, "qapk1", "qapk2");
        add_edge(&mut dag, &indices, "qapk2", "qapk3");
        add_edge(&mut dag, &indices, "qapk3", "qapk4");
        add_edge(&mut dag, &indices, "qapk4", "qapkend");
        let questions = QuestionsDAG { dag };

        let qapi1_index = indices.get("qapi1").unwrap();
        let qapk1_index = indices.get("qapk1").unwrap();

        assert_eq!(questions.find_shallow_node(*qapi1_index), Some(*qapk1_index));

    }

    #[test]
    fn test_deep_chain_only_instants() {
        let mut dag = daggy::Dag::<QANode, ()>::new();
        let mut indices = HashMap::new();
        create_nodes!(dag, indices,
            qapi1: UnderstandingLevel::InstantRecall(2, 3), 
            qapi2: UnderstandingLevel::InstantRecall(2, 3), 
            qapi3: UnderstandingLevel::InstantRecall(2, 3), 
            qapi4: UnderstandingLevel::InstantRecall(2, 3), 
            qapi5: UnderstandingLevel::InstantRecall(2, 3), 
            qapi6: UnderstandingLevel::InstantRecall(2, 3), 
            qapi7: UnderstandingLevel::InstantRecall(2, 3)
        );

        add_edge(&mut dag, &indices, "qapi1", "qapi2");
        add_edge(&mut dag, &indices, "qapi2", "qapi3");
        add_edge(&mut dag, &indices, "qapi3", "qapi4");
        add_edge(&mut dag, &indices, "qapi4", "qapi5");
        add_edge(&mut dag, &indices, "qapi5", "qapi6");
        add_edge(&mut dag, &indices, "qapi6", "qapi7");
        let questions = QuestionsDAG { dag };

        let qapi1_index = indices.get("qapi1").unwrap();

        assert_eq!(questions.find_shallow_node(*qapi1_index), None);

    }

    fn add_edge(dag: &mut daggy::Dag<QANode, ()>, indices: &HashMap<&str, daggy::NodeIndex>, node1: &str, node2: &str) {
        let index1 = indices.get(node1).unwrap();
        let index2 = indices.get(node2).unwrap();
        dag.add_edge(*index1, *index2, ()).unwrap();
    }
}
