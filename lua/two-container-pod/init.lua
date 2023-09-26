-- Based on https://github.com/kubernetes/website/blob/main/content/en/examples/pods/two-container-pod.yaml

local volumes = {
    { name = 'shared-data', emptyDir = {} },
}

local nginx = require('nginx')
local debian = require('debian')

local containers = {
    nginx,
    debian,
}

return {
    apiVersion = 'v1',
    kind = 'Pod',
    metadata = {
        name = 'two-containers',
    },
    spec = {
        restartPolicy = 'Never',
        volumes = volumes,
        containers = containers,
    },
}
