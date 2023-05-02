import React from 'react';
import { useRouter } from 'next/router'
import * as Tabs from '@radix-ui/react-tabs';
import cx from 'classnames';
import * as Breadcrumb from "../../components/breadcrumb";
import { useQuery } from '@tanstack/react-query';
import { App, AttestationView, VoteView, getAttestations, getBlockExtended, getCommittees, getVotes } from '../../pkg/web';
import Root from '../../components/root';
import Link from 'next/link';
import AggregationBits from '../../components/aggregation-bits';
import { Cube } from '@phosphor-icons/react';

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

const Committees = (props: { app: App, slot: string }) => {
  const { isLoading, error, data: committees } = useQuery(
    ["committees", props.slot],
    () => getCommittees(props.app, BigInt(props.slot))
  );
  if (isLoading) {
    return (
      <p>Loading...</p>
    );
  }

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

const Votes = (props: { app: App, slot: string }) => {
  const { isLoading, error, data: votes } = useQuery(
    ["votes", props.slot],
    () => getVotes(props.app, BigInt(props.slot))
  );

  if (isLoading) {
    return (
      <p>Loading...</p>
    );
  }

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

const Attestations = (props: { app: App, slot: string }) => {
  const { isLoading, error, data: attestations } = useQuery(
    ["attestations", props.slot],
    () => getAttestations(props.app, BigInt(props.slot))
  );

  if (isLoading) {
    return (
      <p>Loading...</p>
    );
  }

  return <>
    {
      attestations.map((a, i) => (
        <div key={i}>
          <h3>Attestation {i}</h3>
          <Attestation app={props.app} attestation={a} />
        </div>
      ))
    }
  </>
}

const Attestation = (props: { app: App, attestation: AttestationView }) => {
  const { isLoading, error, data: committees } = useQuery(
    ["committees", props.attestation.slot],
    () => getCommittees(props.app, BigInt(props.attestation.slot))
  );

  if (isLoading) {
    return (
      <p>Loading...</p>
    );
  }

  return (
    <dl>
      <dt>Slot</dt>
      <dd>{props.attestation.slot}</dd>

      <dt>Committee index</dt>
      <dd>{props.attestation.committeeIndex}</dd>

      <dt>Aggregation bits</dt>
      <dd className="flex flex-wrap">
        <AggregationBits bits={props.attestation.aggregationBits} />
      </dd>

      <dt>Validators</dt>
      <dd className="flex flex-wrap">
        <Validators
          validators={committees[props.attestation.committeeIndex].validators}
          aggregationBits={props.attestation.aggregationBits} />
      </dd>

      <dt>Beacon block root</dt>
      <dd><span className="font-mono">{props.attestation.beaconBlockRoot}</span></dd>

      <dt>Source</dt>
      <dd>
        Epoch <Link href={`/epoch/${props.attestation.sourceEpoch}`}>{props.attestation.sourceEpoch}</Link>
        &nbsp;(<span className="font-mono">{props.attestation.sourceRoot}</span>)
      </dd>

      <dt>Target</dt>
      <dd>
        Epoch <Link href={`/epoch/${props.attestation.targetEpoch}`}>{props.attestation.targetEpoch}</Link>
        &nbsp;(<span className="font-mono">{props.attestation.targetRoot}</span>)
      </dd>

      <dt>Signature</dt>
      <dd><span className="font-mono break-words">{props.attestation.signature}</span></dd>
    </dl>
  );
}

export default () => {
  const app = new App(process.env.NEXT_PUBLIC_HOST);
  const router = useRouter();
  const slot = router.query.slot as string;

  if (!slot) {
    return (
      <p>Loading...</p>
    )
  }

  const { isLoading, error, data: block } = useQuery(
    ["block", slot],
    () => getBlockExtended(app, BigInt(slot))
  );

  if (error) {
    return (
      <p>Failed to load {error}</p>
    )
  }

  return (
    <>
      <Breadcrumb.Root>
        <Breadcrumb.Part>
          <Link href="/blocks"><Cube />&nbsp;Blocks</Link>
        </Breadcrumb.Part>
        <Breadcrumb.Part>
          <span>{block && block.slot}</span>
        </Breadcrumb.Part>
      </Breadcrumb.Root>
      <section className="container mx-auto">
        <div className="tabular-data">
          <p>Showing block</p>
          <Tabs.Root defaultValue="overview">
            <Tabs.List>
              <Tabs.Trigger value="overview">Overview</Tabs.Trigger >
              <Tabs.Trigger value="committees">Committees</Tabs.Trigger >
              <Tabs.Trigger value="votes">Votes ({block && block.votesCount})</Tabs.Trigger >
              <Tabs.Trigger value="attestations">Attestations ({block && block.attestationsCount})</Tabs.Trigger >
            </Tabs.List>
            <Tabs.Content value="overview">
              <dl>
                <dt>Epoch</dt>
                <dd>{block && block.epoch}</dd>
                <dt>Slot</dt>
                <dd>{block && block.slot}</dd>
              </dl>
            </Tabs.Content>
            <Tabs.Content value="committees">
              <Committees app={app} slot={slot} />
            </Tabs.Content>
            <Tabs.Content value="votes">
              <Votes app={app} slot={slot} />
            </Tabs.Content>
            <Tabs.Content value="attestations">
              <Attestations app={app} slot={slot} />
            </Tabs.Content>
          </Tabs.Root>
        </div>
      </section>
    </>
  )

}