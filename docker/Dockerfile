FROM debian:trixie-slim

RUN apt-get update \
 && apt-get install -y --no-install-recommends zlib1g libpcap0.8 libfuse3-4 python3 \
 && rm -rf /var/lib/apt/lists/*

COPY docker/plfscgi/ /usr/share/plfscgi/
RUN printf '#!/bin/sh\nPYTHONPATH=/usr/share/plfscgi exec /usr/bin/python3 /usr/share/plfscgi/plfscli "$@"\n' > /usr/local/bin/plfscli \
 && chmod +x /usr/local/bin/plfscli /usr/share/plfscgi/plfscli

COPY target/release/plfsbdev target/release/plfschunkserver target/release/plfsgui target/release/plfsmaster target/release/plfsmetalogger target/release/plfsmount target/release/plfsnetdump /usr/local/sbin/
COPY docker/plfs-entry.sh /usr/local/bin/plfs-entry.sh
RUN chmod +x /usr/local/bin/plfs-entry.sh /usr/local/sbin/plfs*

# role: plfsmaster | plfsmetalogger | plfschunkserver | plfsgui | plfsmount | plfsbdev | plfsnetdump
ENV ROLE=plfsmaster
ENTRYPOINT ["/usr/local/bin/plfs-entry.sh"]
