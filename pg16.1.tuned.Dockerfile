FROM postgres:16.1

# Customizamos:

# - synchronous_commit=off: COMMIT assíncrono para melhorar a performance.
#        obs.: fsync = on, mantendo a garantia de escrita no disco.
#        se desabilitar o fsync, melhora ainda mais, mas aí o risco de perda de dados é grande e eu
#        já considero trapaça.
#   https://postgresqlco.nf/doc/en/param/synchronous_commit/
#   O artigo acima explica bem os tradeoffs.

# - full_page_writes=off: Desabilita a escrita de páginas inteiras para melhorar a performance.
#   https://postgresqlco.nf/doc/en/param/full_page_writes/
#   Em caso de crash o state do banco ainda se mantém hígido.

# - wal_level=minimal: Sets the WAL level to minimal for reduced logging.
#   https://postgresqlco.nf/doc/en/param/wal_level/
#   Fazemos downgrade do WAL para um nível que não suporta replicação, afinal não temos este requisito.
#   Isso acelera bastante a escrita e mantém ainda o necessário para se recuperar de um crash.

# - max_wal_senders=0: Disables WAL senders.
#   https://postgresqlco.nf/doc/en/param/max_wal_senders/
#   Não temos replicação. Este comando apenas garante que teremos 0 processos WAL senders.

# - max_connections=256: Sets the maximum number of connections to 256.
#   Bump agressivo de conexões, visto que demos uma bumpada forte nos pools de conexão do backend também.

CMD ["postgres", "-c", "synchronous_commit=off", "-c", "full_page_writes=off", "-c", "wal_level=minimal", "-c", "max_wal_senders=0", "-c", "max_connections=256"]
