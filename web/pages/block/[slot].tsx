import { useRouter } from 'next/router'
import { TabPanel, useTabs } from 'react-headless-tabs';
import Breadcrumb from "../../components/breadcrumb";
import TabSelector from '../../components/tab-selector';
import { useBlock, useAttestations, useCommitees } from "../../hooks/blocks";

const Validators = ({ validators }) => {
  return validators.map(v => (
    <span className='w-16'>{v}</span>
  ));
}

const Committees = ({ slot }) => {
  const { data: committees } = useCommitees(slot);

  if (!committees) {
    return (
      <p>Loading...</p>
    );
  }

  return committees.map(c => (
    <dl>
      <dt>{c.index}</dt>
      <dd className="flex flex-wrap"><Validators validators={c.validators} /></dd>
    </dl>
  ));
}

const Attestations = ({ slot }) => {
  const { data: attestations } = useAttestations(slot);

  if (!attestations) {
    return (
      <p>Loading...</p>
    );
  }

  return attestations.map(a => (
    <dl>
      <dt>{a.committee_index}</dt>
      <dd className="flex flex-wrap">{a.aggregation_bits.length}</dd>
    </dl>
  ));
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
            Committees
            <Committees slot={slot} />
          </TabPanel>
          <TabPanel hidden={selectedTab !== 'attestations'}>
            Attestations
            <Attestations slot={slot} />
          </TabPanel>
        </div>
      </section>
    </>
  )

}