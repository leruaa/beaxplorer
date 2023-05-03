import React, { Suspense } from 'react';
import { useRouter } from 'next/router'
import * as Tabs from '@radix-ui/react-tabs';
import cx from 'classnames';
import * as Breadcrumb from "../../components/breadcrumb";
import { useQuery } from '@tanstack/react-query';
import { App, AttestationView, BlockPaths, VoteView, getAttestations, getBlockExtended, getBlockPaths, getCommittees, getVotes } from '../../pkg/web';
import Root from '../../components/root';
import Link from 'next/link';
import AggregationBits from '../../components/aggregation-bits';
import { Cube } from '@phosphor-icons/react';
import { useBuffer } from '../../hooks/data';

const Validators = (props: { validators: number[], aggregationBits?: boolean[] }) => {
  if (!props.validators) {
    return (
      <p>Loading...</p>
    );
  }

  let validators = props.validators.map(
    (v, i) => (
      <span key={v} className={cx({ 'w-16': true, "text-gray-400": props.aggregationBits && props.aggregationBits.length > 0 && !props.aggregationBits[i] })}>
        {v}
      </span>
    )
  );

  return <>{validators}</>
}

type ModelsProps = { slot: bigint, path: string };

const Committees = ({ slot, path }: ModelsProps) => {
  const { data: committees } = useQuery({
    queryKey: [path],
    queryFn: () => useBuffer(slot, path).then(committeesBuffer => getCommittees(committeesBuffer.buffer)),
    suspense: true
  });

  return <>
    {
      committees.map(c => (
        <dl key={c.index}>
          <dt>{c.index}</dt>
          <dd className="flex flex-wrap"><Validators validators={c.validators} /></dd>
        </dl>
      ))
    }
  </>
}

const Votes = ({ slot, path }: ModelsProps) => {
  const { data: votes } = useQuery({
    queryKey: [path],
    queryFn: () => useBuffer(slot, path).then(votesBuffer => getVotes(votesBuffer.buffer)),
    suspense: true
  });

  return <>
    {
      votes.map((v, i) => (
        <Vote key={i} vote={v} />
      ))
    }
  </>
}

const Vote = (props: { vote: VoteView }) => {
  return (
    <dl>
      <dt>Slot</dt>
      <dd>{props.vote.slot}</dd>

      <dt>Committee index</dt>
      <dd>{props.vote.committeeIndex}</dd>

      <dt>Included in block</dt>
      <dd>{props.vote.includedIn}</dd>

      <dt>Validators</dt>
      <dd className="flex flex-wrap"><Validators validators={props.vote.validators} /></dd>
    </dl>
  );
}

type AttestationsProps = { slot: bigint, paths: BlockPaths };

const Attestations = ({ slot, paths }: AttestationsProps) => {
  const { data: attestations } = useQuery({
    queryKey: [paths.attestations],
    queryFn: () => useBuffer(slot, paths.attestations).then(attestationsBuffer => getAttestations(attestationsBuffer.buffer)),
    suspense: true
  });

  return <>
    {
      attestations.map((a, i) => (
        <div key={i}>
          <h3>Attestation {i}</h3>
          <Attestation slot={slot} attestation={a} committeesPath={paths.committees} />
        </div>
      ))
    }
  </>
}

type AttestationProps = { slot: bigint, attestation: AttestationView, committeesPath: string };

const Attestation = ({ slot, attestation, committeesPath }: AttestationProps) => {
  const { data: committees } = useQuery({
    queryKey: [committeesPath],
    queryFn: () => useBuffer(slot, committeesPath).then(committeesBuffer => getCommittees(committeesBuffer.buffer)),
    suspense: true
  });


  return (
    <dl>
      <dt>Slot</dt>
      <dd>{attestation.slot}</dd>

      <dt>Committee index</dt>
      <dd>{attestation.committeeIndex}</dd>

      <dt>Aggregation bits</dt>
      <dd className="flex flex-wrap">
        <AggregationBits bits={attestation.aggregationBits} />
      </dd>

      <dt>Validators</dt>
      <dd className="flex flex-wrap">
        <Validators
          validators={committees[attestation.committeeIndex].validators}
          aggregationBits={attestation.aggregationBits} />
      </dd>

      <dt>Beacon block root</dt>
      <dd><span className="font-mono">{attestation.beaconBlockRoot}</span></dd>

      <dt>Source</dt>
      <dd>
        Epoch <Link href={`/epoch/${attestation.sourceEpoch}`}>{attestation.sourceEpoch}</Link>
        &nbsp;(<span className="font-mono">{attestation.sourceRoot}</span>)
      </dd>

      <dt>Target</dt>
      <dd>
        Epoch <Link href={`/epoch/${attestation.targetEpoch}`}>{attestation.targetEpoch}</Link>
        &nbsp;(<span className="font-mono">{attestation.targetRoot}</span>)
      </dd>

      <dt>Signature</dt>
      <dd><span className="font-mono break-words">{attestation.signature}</span></dd>
    </dl>
  );
}

const Block = ({ slot }: { slot: bigint }) => {
  const app = new App(process.env.NEXT_PUBLIC_HOST);
  const blockPaths = getBlockPaths(app, slot);

  const { data: block } = useQuery({
    queryKey: [blockPaths.block, blockPaths.blockExtended],
    queryFn: () => {
      return Promise.all([useBuffer(slot, blockPaths.block), useBuffer(slot, blockPaths.blockExtended)])
        .then(([epochBuffer, epochExtendedBuffer]) =>
          getBlockExtended(epochBuffer.buffer, epochExtendedBuffer.buffer, slot)
        )
    },
    suspense: true
  });

  return (
    <>
      <section className="container mx-auto">
        <div className="tabular-data">
          <p>Showing block</p>
          <Tabs.Root defaultValue="overview">
            <Tabs.List>
              <Tabs.Trigger value="overview">Overview</Tabs.Trigger >
              <Tabs.Trigger value="committees">Committees</Tabs.Trigger >
              <Tabs.Trigger value="votes">Votes ({block.votesCount})</Tabs.Trigger >
              <Tabs.Trigger value="attestations">Attestations ({block.attestationsCount})</Tabs.Trigger >
            </Tabs.List>
            <Tabs.Content value="overview">
              <dl>
                <dt>Epoch</dt>
                <dd>{block.epoch}</dd>
                <dt>Slot</dt>
                <dd>{block.slot}</dd>
              </dl>
            </Tabs.Content>
            <Tabs.Content value="committees">
              <Suspense fallback={<Loading />}>
                <Committees slot={slot} path={blockPaths.committees} />
              </Suspense>
            </Tabs.Content>
            <Tabs.Content value="votes">
              <Suspense fallback={<Loading />}>
                <Votes slot={slot} path={blockPaths.votes} />
              </Suspense>
            </Tabs.Content>
            <Tabs.Content value="attestations">
              <Suspense fallback={<Loading />}>
                <Attestations slot={slot} paths={blockPaths} />
              </Suspense>
            </Tabs.Content>
          </Tabs.Root>
        </div>
      </section>
    </>
  )
}

export default () => {

  const router = useRouter();
  const slot = router.query.slot as string;

  return (
    <>
      <Breadcrumb.Root>
        <Breadcrumb.Part>
          <Link href="/blocks"><Cube />&nbsp;Blocks</Link>
        </Breadcrumb.Part>
        <Breadcrumb.Part>
          <span>{slot}</span>
        </Breadcrumb.Part>
      </Breadcrumb.Root>
      {slot ? (
        <Suspense fallback={<Loading />}>
          <Block slot={BigInt(slot)} />
        </Suspense>
      ) : (
        <Loading />
      )}
    </>
  )
}

const Loading = () => {
  return (<p>Loading...</p>)
}