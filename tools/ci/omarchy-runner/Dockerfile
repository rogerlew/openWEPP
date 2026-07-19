FROM ubuntu:24.04@sha256:4fbb8e6a8395de5a7550b33509421a2bafbc0aab6c06ba2cef9ebffbc7092d90

ARG RUNNER_VERSION=2.335.1
ARG RUNNER_SHA256=4ef2f25285f0ae4477f1fe1e346db76d2f3ebf03824e2ddd1973a2819bf6c8cf
ARG RUST_VERSION=1.92.0
ARG RUSTUP_INIT_SHA256=6c30b75a75b28a96fd913a037c8581b580080b6ee9b8169a3c0feb1af7fe8caf

ENV DEBIAN_FRONTEND=noninteractive \
    CARGO_HOME=/opt/cargo \
    RUSTUP_HOME=/opt/rustup \
    PATH=/opt/cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
       ca-certificates curl git jq build-essential pkg-config libssl-dev \
       python3 python3-venv python3-pip unzip xz-utils zstd passwd \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --create-home --uid 10001 --shell /bin/bash runner \
    && mkdir -p /opt/actions-runner /runner-state /runner-work /cache/cargo /cache/target \
    && chown -R runner:runner /opt/actions-runner /runner-state /runner-work /cache

WORKDIR /opt/actions-runner
RUN curl --fail --location --silent --show-error \
      "https://github.com/actions/runner/releases/download/v${RUNNER_VERSION}/actions-runner-linux-x64-${RUNNER_VERSION}.tar.gz" \
      --output /tmp/actions-runner.tar.gz \
    && echo "${RUNNER_SHA256}  /tmp/actions-runner.tar.gz" | sha256sum --check --strict \
    && tar -xzf /tmp/actions-runner.tar.gz \
    && rm /tmp/actions-runner.tar.gz \
    && ./bin/installdependencies.sh

RUN curl --proto '=https' --tlsv1.2 --silent --show-error --fail \
      https://sh.rustup.rs --output /tmp/rustup-init.sh \
    && echo "${RUSTUP_INIT_SHA256}  /tmp/rustup-init.sh" | sha256sum --check --strict \
    && sh /tmp/rustup-init.sh -y --profile minimal --default-toolchain "${RUST_VERSION}" \
    && rm /tmp/rustup-init.sh \
    && rustup component add rustfmt clippy \
    && cargo install cargo-nextest --locked --version 0.9.138 \
    && cargo install cargo-deny --locked --version 0.19.6 \
    && cargo install cargo-llvm-cov --locked --version 0.8.7 \
    && cargo install cargo-crap --locked --version 0.2.2 \
    && rm -rf /opt/cargo/registry/src /opt/cargo/git

COPY markdown-doc /usr/local/bin/markdown-doc
RUN echo "265cdb2d3d6d5e7e98aac18f3bd1041278106d41ba5ff1cf23638381d346ea34  /usr/local/bin/markdown-doc" \
      | sha256sum --check --strict \
    && chmod 0755 /usr/local/bin/markdown-doc

COPY entrypoint.sh /usr/local/bin/openwepp-runner-entrypoint
COPY job-completed-hook.sh /usr/local/bin/openwepp-job-completed-hook.sh
RUN chmod 0755 /usr/local/bin/openwepp-runner-entrypoint \
      /usr/local/bin/openwepp-job-completed-hook.sh \
    && chown root:root /usr/local/bin/openwepp-runner-entrypoint \
      /usr/local/bin/openwepp-job-completed-hook.sh \
    && chmod -R go-w /opt/actions-runner /opt/cargo /opt/rustup

ENV CARGO_HOME=/cache/cargo \
    CARGO_TARGET_DIR=/cache/target \
    RUSTUP_HOME=/opt/rustup \
    RUSTUP_TOOLCHAIN=1.92.0-x86_64-unknown-linux-gnu \
    RUNNER_ALLOW_RUNASROOT=0 \
    ACTIONS_RUNNER_HOOK_JOB_COMPLETED=/usr/local/bin/openwepp-job-completed-hook.sh

USER runner
WORKDIR /runner-state
ENTRYPOINT ["/usr/local/bin/openwepp-runner-entrypoint"]
CMD ["run"]
