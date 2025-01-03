use leptos::document;


pub(crate) fn prepend_relative_url(relative_url: &str) -> String {
    let document_head = document().head().unwrap();
    let meta_tags = document_head.get_elements_by_tag_name("meta");
    let mut domain_path = "".to_owned();
    for i in 0..meta_tags.length() {
        let Some(meta_tag) = meta_tags.item(i) else {break};
        let Some(name_attr) = meta_tag.get_attribute("name") else {continue};
        if name_attr != "domain-path" {continue};
        domain_path = meta_tag.get_attribute("content").unwrap();
    }
    domain_path + relative_url
}

