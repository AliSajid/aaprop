FROM gitpod/workspace-full:2022-10-05-23-28-15@sha256:3a72ddc1804ad501cd2cf681279f4319698a17c715133a73a576ceb25d675067

# Install your tools here

RUN brew install \
    tmux \
    bat \
    tldr \
    ripgrep \
    exa \
    fzf \
    gitleaks \
    markdownlint-cli \
    direnv \
    commitizen
