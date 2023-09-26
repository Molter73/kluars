local nginx_mounts = {
    { name = 'shared-data', mountPath = '/usr/share/nginx/html' },
}

return {
    name = 'nginx',
    image = 'nginx',
    volumeMounts = nginx_mounts,
}
