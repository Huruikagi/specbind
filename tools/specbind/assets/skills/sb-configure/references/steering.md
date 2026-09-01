# Steering configuration

Use Steering for durable project knowledge that future work should consume.
Do not use it for transient scope, current task state, SpecBind's own metadata,
or a policy that belongs in a Rule or adapter.

Route bootstrap, synchronization, addition, and document maintenance through
the current `sb-steering` workflow. Give it the maintainer's requested
outcome and the configuration context already established; do not duplicate its
authoring procedure here.

After it finishes, resume configuration ownership:

```sh
specbind steering list
specbind configuration show
```

Inspect the actual inventory and diff. Steering is not Gate evidence and its
edit approves nothing. Nevertheless, offer synchronization or revalidation of
active work whose assumptions may now conflict with the changed durable
guidance. Never mutate Requirements, Design, or lifecycle state merely to make
them agree.
