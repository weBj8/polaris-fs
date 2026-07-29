FROM debian:trixie-slim

RUN apt-get update \
 && apt-get install -y --no-install-recommends zlib1g libpcap0.8 libfuse3-4 \
 && rm -rf /var/lib/apt/lists/*

COPY dist/ /usr/local/sbin/
COPY docker/mfs-entry.sh /usr/local/bin/mfs-entry.sh
RUN chmod +x /usr/local/bin/mfs-entry.sh /usr/local/sbin/mfs*

# role: mfsmaster | mfsmetalogger | mfschunkserver | mfsgui | mfsmount | mfsbdev | mfsnetdump
ENV ROLE=mfsmaster
ENTRYPOINT ["/usr/local/bin/mfs-entry.sh"]
