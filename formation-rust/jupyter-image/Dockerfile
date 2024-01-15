FROM buildpack-deps:20.04-curl
LABEL maintainer="Laurent Wouters <lwouters@cenotelie.fr>" vendor="Cénotélie Opérations SAS" description="Cénotélie - Formation Rust - Jupyter Kernel"

# Custom Home
RUN mkdir /home/builder
ENV HOME=/home/builder

# Add support for Jupyter
RUN apt-get update && \
        DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
                pkg-config \
                build-essential \
                cmake \
                libzmq3-dev \
                jupyter-notebook \
                python3-pip \
                python-setuptools \
                pandoc \
                texlive-xetex \
                texlive-fonts-recommended \
                texlive-plain-generic 
RUN     pip3 install --upgrade pip && \
        pip3 install wheel && \
        pip3 install jupyter_contrib_nbextensions && \
        pip3 install jupyter_nbextensions_configurator && \
        jupyter contrib nbextension install --user && \
        jupyter nbextensions_configurator enable --user && \
        pip3 install jupyterthemes && \
        jt -t onedork

# Add support for Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="${HOME}/.cargo/bin:${PATH}"
ENV RUST_BACKTRACE=1
# ENV RUST_SRC_PATH="$(rustc --print sysroot)/lib/rustlib/src/rust/src"
RUN mkdir /home/builder/.cargo/registry
RUN rustup component add rust-src

# Install EvCxR Jupyter Kernel
RUN     cargo install evcxr_jupyter && \
        evcxr_jupyter --install
