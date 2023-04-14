import React, { useMemo } from 'react';
import { useRouter } from 'next/router'
import { TabPanel, useTabs } from 'react-headless-tabs';
import cx from 'classnames';
import Breadcrumb from "../../components/breadcrumb";
import TabSelector from '../../components/tab-selector';
import { useQuery } from '@tanstack/react-query';
import { App, AttestationView, getAttestations, getBlockExtended, getCommittees, getVotes } from '../../pkg/web';

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
  debugger;
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

const Vote = ({ vote }) => {
  return (
    <dl>
      <dt>Slot</dt>
      <dd>{vote.slot}</dd>

      <dt>Committee index</dt>
      <dd>{vote.committee_index}</dd>

      <dt>Validators</dt>
      <dd className="flex flex-wrap"><Validators validators={vote.validators} /></dd>
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

      <dt>Validators</dt>
      <dd className="flex flex-wrap">
        <Validators
          validators={committees[props.attestation.committeeIndex].validators}
          aggregationBits={props.attestation.aggregationBits} />
      </dd>

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

  const [selectedTab, setSelectedTab] = useTabs([
    'overview',
    'committees',
    'votes',
    'attestations'
  ]);

  if (error) {
    return (
      <p>Failed to load {error}</p>
    )
  }

  return (
    <>
      <Breadcrumb breadcrumb={{ parts: [{ text: "Blocks", icon: "clock" }] }} />
      <section className="container mx-auto">
        <div className="tabular-data">
          <p>Showing block</p>

          <nav>
            <TabSelector
              isActive={selectedTab === 'overview'}
              onClick={() => setSelectedTab('overview')}
            >
              Overview
            </TabSelector>

            <TabSelector
              isActive={selectedTab === 'committees'}
              onClick={() => setSelectedTab('committees')}
            >
              Committees
            </TabSelector>


            <TabSelector
              isActive={selectedTab === 'votes'}
              onClick={() => setSelectedTab('votes')}
            >
              Votes ({block && block.votes_count})
            </TabSelector>

            <TabSelector
              isActive={selectedTab === 'attestations'}
              onClick={() => setSelectedTab('attestations')}
            >
              Attestations ({block && block.attestations_count})
            </TabSelector>
          </nav>

          <TabPanel hidden={selectedTab !== 'overview'}>
            <dl>
              <dt>Epoch</dt>
              <dd>{block && block.epoch}</dd>
              <dt>Slot</dt>
              <dd>{block && block.slot}</dd>
            </dl>
          </TabPanel>

          <TabPanel hidden={selectedTab !== 'committees'}>
            <Committees app={app} slot={slot} />
          </TabPanel>

          <TabPanel hidden={selectedTab !== 'votes'}>
            <Votes app={app} slot={slot} />
          </TabPanel>

          <TabPanel hidden={selectedTab !== 'attestations'}>
            <Attestations app={app} slot={slot} />
          </TabPanel>
        </div>
      </section>
    </>
  )

}