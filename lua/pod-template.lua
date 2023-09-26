local name = name or 'nginx'
local port = tonumber(port) or 80

local metadata = {
    name = name,
}

local nginx = {
    name = name,
    image = 'nginx:1.14.2',
    ports = {
        { containerPort = port, }
    },
}

local spec = {
    containers = {
        nginx,
    },
}

return {
    apiVersion = 'v1',
    kind = 'Pod',
    metadata = metadata,
    spec = spec,
}
