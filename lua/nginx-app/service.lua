local metadata = {
    name = 'my-nginx-svc',
    labels = {
        app = 'nginx',
    },
}

local spec = {
    type = 'LoadBalancer',
    ports = {
        { port = 80 },
    },
    selector = {
        app = 'nginx'
    },
}

return {
    apiVersion = 'v1',
    kind = 'Service',
    metadata = metadata,
    spec = spec,
}
