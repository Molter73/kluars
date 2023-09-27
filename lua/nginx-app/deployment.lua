local metadata = {
    name = 'my-nginx',
    labels = {
        app = 'nginx',
    },
}

local containers = {
    {
        name = 'nginx',
        image = 'nginx:1.14.2',
        ports = {
            { containerPort = 80 },
        },
    },
}

local template = {
    metadata = {
        labels = {
            app = 'nginx',
        },
    },
    spec = {
        containers = containers,
    },
}

local spec = {
    replicas = 3,
    selector = {
        matchLabels = {
            app = 'nginx',
        },
    },
    template = template,
}

return {
    apiVersion = 'apps/v1',
    kind = 'Deployment',
    metadata = metadata,
    spec = spec,
}

