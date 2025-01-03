#!/usr/bin/env bash
set -e

appName="leptos-tutorial"
stylePrefix="output"
styleFormat="css"

# Extract build version
indexJsFile=$(find "${TRUNK_STAGING_DIR}" -iname "${appName}-*.js")
echo "Extracting build version from file: ${indexJsFile}"
regex="(.*)${appName}-(.*).js"
_src="${indexJsFile}"
while [[ "${_src}" =~ ${regex} ]]; do
    buildVersion="${BASH_REMATCH[2]}"
    _i=${#BASH_REMATCH}
    _src=${_src:_i}
done
if [ -z "${buildVersion}" ]; then
    echo "Could not determine build version!"
    exit 1
fi
echo "Build-Version is: ${buildVersion}"

# Replace placeholder in service-worker.js
serviceWorkerJsFile=$(find "${TRUNK_STAGING_DIR}" -iname "service_worker.js")
echo "Replacing {{buildVersion}} placeholder in: ${serviceWorkerJsFile}"
sed "s/{{buildVersion}}/${buildVersion}/g" "${serviceWorkerJsFile}" > "${serviceWorkerJsFile}.modified"
mv -f "${serviceWorkerJsFile}.modified" "${serviceWorkerJsFile}"

# Extract CSS build version
indexJsFile=$(find "${TRUNK_STAGING_DIR}" -iname "${stylePrefix}-*.${styleFormat}")
echo "Extracting style build version from file: ${indexJsFile}"
regex="(.*)${stylePrefix}-(.*).${styleFormat}"
_src="${indexJsFile}"
while [[ "${_src}" =~ ${regex} ]]; do
    cssBuildVersion="${BASH_REMATCH[2]}"
    _i=${#BASH_REMATCH}
    _src=${_src:_i}
done
if [ -z "${cssBuildVersion}" ]; then
    echo "Could not determine style build version!"
    exit 1
fi
echo "CSS Build-Version is: ${cssBuildVersion}"

# Replace placeholder in service-worker.js
serviceWorkerJsFile=$(find "${TRUNK_STAGING_DIR}" -iname "service_worker.js")
echo "Replacing {{cssBuildVersion}} placeholder in: ${serviceWorkerJsFile}"
sed "s/{{cssBuildVersion}}/${cssBuildVersion}/g" "${serviceWorkerJsFile}" > "${serviceWorkerJsFile}.modified"
mv -f "${serviceWorkerJsFile}.modified" "${serviceWorkerJsFile}"