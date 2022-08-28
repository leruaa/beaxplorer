
import { useMemo } from 'react';
import useSWR from 'swr';
import { Blocks, BlocksMeta } from "../pkg/web";

function useBlocksFetcher() {
  return useMemo(() => Blocks.build("http://localhost:3000"), []);
}

export function useBlock(slot: string) {
  const fetcher = useBlocksFetcher();
  return useSWR(
    slot !== undefined ? ["block", slot] : null,
    (_, s) => fetcher.then(blocks => blocks.get(BigInt(s)))
  );
}

export function usePagedBlocks(pageIndex: number, pageSize: number, sortId: string, sortDesc: boolean, meta: BlocksMeta) {
  const fetcher = useBlocksFetcher();
  return useSWR(
    ["blocks", pageIndex, pageSize, sortId, sortDesc],
    (_, pageIndex, pageSize, sortId, sortDesc) => fetcher.then(blocks => blocks.page(pageIndex, pageSize, sortId, sortDesc))
  );
}

export function useCommittees(slot: string) {
  const fetcher = useBlocksFetcher();
  return useSWR(
    slot !== undefined ? ["committees", slot] : null,
    (_, s) => fetcher.then(blocks => blocks.committees(BigInt(s)))
  );
}

export function useVotes(slot: string) {
  const fetcher = useBlocksFetcher();
  return useSWR(
    slot !== undefined ? ["votes", slot] : null,
    (_, s) => fetcher.then(blocks => blocks.votes(BigInt(s)))
  );
}


export function useAttestations(slot: string) {
  const fetcher = useBlocksFetcher();
  return useSWR(
    slot !== undefined ? ["attestations", slot] : null,
    (_, s) => fetcher.then(blocks => blocks.attestations(BigInt(s)))
  );
}