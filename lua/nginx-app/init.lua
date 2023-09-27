-- Taken from https://github.com/kubernetes/website/blob/main/content/en/examples/application/nginx-app.yaml

local service = require('service')
local deployment = require('deployment')

return {
    service,
    deployment,
}
