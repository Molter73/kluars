--[[
apiVersion: v1
kind: Pod
metadata:
  name: nginx
spec:
  containers:
  - name: nginx
    image: nginx:1.14.2
    ports:
    - containerPort: 80
--]]

local metadata = {
    name = 'nginx',
}

local nginx = {
    name = 'nginx',
    image = 'nginx:1.14.2',
    ports = {
        { containerPort = 80, }
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
