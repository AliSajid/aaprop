FROM gitpod/workspace-full:2022-10-05-23-28-15

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
