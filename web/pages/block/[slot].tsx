import React, { useMemo } from 'react';
import { useRouter } from 'next/router'
import { TabPanel, useTabs } from 'react-headless-tabs';
import cx from 'classnames';
import Breadcrumb from "../../components/breadcrumb";
import TabSelector from '../../components/tab-selector';
import { useQuery } from '@tanstack/react-query';
import { App, getAttestations, getBlockExtended, getCommittees, getVotes } from '../../pkg/web';

const Validators = ({ validators, aggregation_bits = [] }) => {
  if (!validators) {
    return (
      <p>Loading...</p>
    );
  }

  return validators.map((v, i) => (
    <span key={v} className={cx({ 'w-16': true, "text-gray-400": aggregation_bits.length > 0 && !aggregation_bits[i] })}>
      {v}
    </span>
  ));
}

const Committees = ({ app, slot }) => {
  debugger;
  const { isLoading, error, data: committees } = useQuery(
    ["committees", slot],
    () => getCommittees(app, BigInt(slot))
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

const Votes = ({ app, slot }) => {
  const { isLoading, error, data: votes } = useQuery(
    ["votes", slot],
    () => getVotes(app, BigInt(slot))
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

const Attestations = ({ app, slot }) => {
  const { isLoading, error, data: attestations } = useQuery(
    ["attestations", slot],
    () => getAttestations(app, BigInt(slot))
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
          <Attestation app={app} attestation={a} />
        </div>
      ))
    }
  </>
}

const Attestation = ({ app, attestation }) => {
  const { isLoading, error, data: committees } = useQuery(
    ["committees", attestation.slot],
    () => getCommittees(app, BigInt(attestation.slot))
  );

  if (isLoading) {
    return (
      <p>Loading...</p>
    );
  }

  return (
    <dl>
      <dt>Slot</dt>
      <dd>{attestation.slot}</dd>

      <dt>Committee index</dt>
      <dd>{attestation.committee_index}</dd>

      <dt>Validators</dt>
      <dd className="flex flex-wrap">
        <Validators
          validators={committees[attestation.committee_index].validators}
          aggregation_bits={attestation.aggregation_bits} />
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