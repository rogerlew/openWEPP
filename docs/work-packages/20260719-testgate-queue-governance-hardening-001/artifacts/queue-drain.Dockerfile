FROM openwepp-actions-runner:2.335.1

USER root
COPY reject-job.sh /usr/local/bin/openwepp-reject-job.sh
RUN chmod 0555 /usr/local/bin/openwepp-reject-job.sh \
    && chown root:root /usr/local/bin/openwepp-reject-job.sh

ENV ACTIONS_RUNNER_HOOK_JOB_STARTED=/usr/local/bin/openwepp-reject-job.sh

USER runner
