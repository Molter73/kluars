local debian_mounts = {
    { name = 'shared-data', mountPath = '/pod-data' },
}

return {
    name = 'debian-container',
    image = 'debian',
    volumeMounts = debian_mounts,
    command = { '/bin/sh' },
    args = { '-c', 'echo Hello from the debian container > /pod-data/index.html' },
}
