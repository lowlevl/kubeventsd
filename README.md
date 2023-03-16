# kubeventsd: A small k8s event watcher daemon [![Continuous integration](https://github.com/Nurrl/kubeventsd/actions/workflows/ci.yaml/badge.svg)](https://github.com/Nurrl/kubeventsd/actions/workflows/ci.yaml)

This project has been started as a need to monitor one of my kubernetes
cluster, and notably the `Job`s failures, but can be used to monitor any kind of `core/v1:Event` in the cluster.

# Configuration

The project configuration is loaded from a `.yaml` file, the path can be provided using the `KUBEVENTSD_CONFIG_PATH` environment variable.

The configuration is separated into two parts:

## `events:` part

This defines filters on the events and to which `senders` the event should be sent if there is a match.

- `reason` (_list[str]_): A list of **reasons** that must match the `core/v1:Event` to be forwarded.
- `type` (_list[str]_): A list of **types** that must match the `core/v1:Event` to be forwarded.
- `namespace` (_list[str]_): A list of **namespaces** that must match the `core/v1:Event` to be forwarded.

When a field is ommited, it's considered to be a **wildcard** match.

### Example:

```yaml
events:
  # This forwards all events.
  - to: [sender0]
  # This forwards all events with 'Started' or 'Completed' as a `reason`.
  - reason: [Started, Completed]
    to: [sender1]
  # This forward all events with 'Killed' or 'BackoffLimitExceeded' as a `reason` in the 'default' `namespace`.
  - reason: [Killed, BackoffLimitExceeded]
    namespace: [default]
    to: [sender2]
```

## `senders:` part

This defines the targets to which the event could be sent.

### Example:

```yaml
senders:
  - name: sender0
    spec:
      kind: Webhook
      url: https://hook.example.com
  - name: sender1
    spec:
      kind: Matrix
      template: |
        {% comment %}
        This contains a liquid-rs template to format the event before sending
        {% endcomment %}
      homeserverUrl: https://matrix.org
      userId: "@example:matrix.org"
      passwordEnv: MATRIX_PASSWORD
      roomId: "!***:matrix.org"
```

### The `Webhook` sender

This sender will `POST` the entire kubernetes `core/v1:Event` object as a JSON body to the provided URL.

- `url` (_URL_): The URL of the webhook.

### The `Matrix` sender

This sender will send the `core/v1:Event` as a templatized message to the specified Matrix room using the provided credentials.

- `template` (_liquid template_): The template for the message rendering, with all `core/v1:Event` accessibles.
- `homeserverUrl` (_URL_): The URL of the Matrix server that is the homeserver of the the provided user.
- `userId` (_Matrix UserId_): The user's identifier to log in the homeserver.
- `passwordEnv` (_str_): The **name** of the **environment variable** that contains the password.
- `roomId` (_Matrix RoomId_): The identifier of the room on which the message should be sent.
