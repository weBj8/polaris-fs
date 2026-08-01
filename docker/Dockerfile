FROM debian:trixie-slim AS fuse

ARG FUSE_VERSION=3.18.2
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates curl gcc libc6-dev meson ninja-build pkg-config python3 \
 && curl -fsSL "https://github.com/libfuse/libfuse/releases/download/fuse-${FUSE_VERSION}/fuse-${FUSE_VERSION}.tar.gz" | tar xz \
 && meson setup "fuse-${FUSE_VERSION}/build" "fuse-${FUSE_VERSION}" -Dexamples=false -Dtests=false -Dutils=false \
 && ninja -C "fuse-${FUSE_VERSION}/build" \
 && DESTDIR=/out ninja -C "fuse-${FUSE_VERSION}/build" install

FROM debian:trixie-slim

RUN apt-get update \
 && apt-get install -y --no-install-recommends zlib1g libpcap0.8 python3 \
 && rm -rf /var/lib/apt/lists/*
COPY --from=fuse /out/usr/local/ /usr/local/
RUN echo /usr/local/lib64 > /etc/ld.so.conf.d/libfuse3.conf && ldconfig

COPY docker/plfscgi/ /usr/share/plfscgi/
RUN printf '#!/bin/sh\nPYTHONPATH=/usr/share/plfscgi exec /usr/bin/python3 /usr/share/plfscgi/plfscli "$@"\n' > /usr/local/bin/plfscli \
 && chmod +x /usr/local/bin/plfscli /usr/share/plfscgi/plfscli

COPY target/release/plfsbdev target/release/plfschunkserver target/release/plfsgui target/release/plfsmaster target/release/plfsmetalogger target/release/plfsmount target/release/plfsnetdump /usr/local/sbin/
COPY docker/plfs-entry.sh /usr/local/bin/plfs-entry.sh
RUN chmod +x /usr/local/bin/plfs-entry.sh /usr/local/sbin/plfs*

# role: plfsmaster | plfsmetalogger | plfschunkserver | plfsgui | plfsmount | plfsbdev | plfsnetdump
ENV ROLE=plfsmaster
ENTRYPOINT ["/usr/local/bin/plfs-entry.sh"]
