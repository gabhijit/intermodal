# Intermodal

Container handling in Rust.

# Introduction

The goal is to implement functionality that can be used to handle Containers in Rust. More concretely -

1. Functionality to handle Container Images (Docker and OCI)
2. OCI Compliant Container Runtime
3. CRI Server, so this can run on a kubelet.
4. Tools/Utils that can be used directly.

The Goal is to make something that tools like  [skopeo](), [podman]() and [runc]() achieve but implemented in Rust.

# Status

This is not even a `v0.1.0` yet, some functionality to 'inspect' docker images along the lines of `skopeo inspect` is present so far and a few test cases.

# Roadmap

The broad plan to implement the following -

1. Image Inspect and Image Pull (So that a `rootfs` can be created.)
2. Basic Runtime support that will utilize above `rootfs` to bring up a container.
3. Add features like `cgroup`, `seccomp` etc.
4. Runtime with support for VMs (Using [rust-vmm]()).
5. Front-end CRI server and other machinery needed (like `CNI` support etc.) to make it run with `crictl`.
