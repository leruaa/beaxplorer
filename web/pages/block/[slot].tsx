import React, { useState, useMemo, useEffect } from 'react';
import { useRouter } from 'next/router'
import { TabPanel, useTabs } from 'react-headless-tabs';
import Breadcrumb from "../../components/breadcrumb";
import TabSelector from '../../components/tab-selector';
import { useBlock, useAttestations, useVotes, useCommittees } from "../../hooks/blocks";

const Validators = ({ validators }) => {
  if (!validators) {
    return (
      <p>Loading...</p>
    );
  }

  return validators.map(v => (
    <span key={v} className='w-16'>{v}</span>
  ));
}

const Committees = ({ slot }) => {
  const { data: committees } = useCommittees(slot);

  if (!committees) {
    return (
      <p>Loading...</p>
    );
  }

  return committees.map(c => (
    <dl key={c.index}>
      <dt>{c.index}</dt>
      <dd className="flex flex-wrap"><Validators validators={c.validators} /></dd>
    </dl>
  ));
}

const Votes = ({ slot }) => {
  const { data: votes } = useVotes(slot);

  if (!votes) {
    return (
      <p>Loading...</p>
    );
  }

  return votes.map((v, i) => (
    <Vote key={i} vote={v} />
  ));
}

const Vote = ({ vote }) => {
  const { data: committees } = useCommittees(vote.slot);
  const validators = useMemo(() => committees ? committees[vote.committee_index].validators : undefined, [committees]);

  if (!validators) {
    return (
      <p>Loading...</p>
    );
  }

  return (
    <dl>
      <dt>Slot</dt>
      <dd>{vote.slot}</dd>

      <dt>Committee index</dt>
      <dd>{vote.committee_index}</dd>

      <dt>Validators</dt>
      <dd className="flex flex-wrap"><Validators validators={validators} /></dd>
    </dl>
  );
}

const Attestations = ({ slot }) => {
  const { data: attestations } = useAttestations(slot);

  if (!attestations) {
    return (
      <p>Loading...</p>
    );
  }

  return attestations.map((a, i) => (
    <div key={i}>
      <h3>Attestation {i}</h3>
      <Attestation attestation={a} />
    </div>
  ));
}

const Attestation = ({ attestation }) => {
  const { data: committees } = useCommittees(attestation.slot);

  if (!committees) {
    return (
      <p>Loading... {attestation.slot}</p>
    );
  }

  return (
    <dl>
      <dt>Slot</dt>
      <dd>{attestation.slot}</dd>

      <dt>Committee index</dt>
      <dd>{attestation.committee_index}</dd>

      <dt>Validators</dt>
      <dd className="flex flex-wrap"><Validators validators={committees[attestation.committee_index].validators} /></dd>

    </dl>
  );
}

export default () => {
  const router = useRouter();
  const { slot } = router.query;
  const { data: block, error } = useBlock(slot as string);

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
              Votes (0)
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
            <Committees slot={slot} />
          </TabPanel>

          <TabPanel hidden={selectedTab !== 'votes'}>
            <Votes slot={slot} />
          </TabPanel>

          <TabPanel hidden={selectedTab !== 'attestations'}>
            <Attestations slot={slot} />
          </TabPanel>
        </div>
      </section>
    </>
  )

}