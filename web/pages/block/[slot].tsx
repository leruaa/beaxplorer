import { useRouter } from 'next/router'
import { TabPanel, useTabs } from 'react-headless-tabs';
import Breadcrumb from "../../components/breadcrumb";
import TabSelector from '../../components/tab-selector';
import { Blocks } from "../../pkg";

export async function getServerSideProps(context) {
  const wasmModule = await import('../../pkg');
  const blocks = await Blocks.build("http://localhost:3000");
  return {
    props: {
      block: await blocks.get(context.params.slot),
      committees: await blocks.committees(context.params.slot),
    }
  }
}

const Validators = ({ validators }) => {
  return validators.map(v => (
    <span className='w-16'>{v}</span>
  ));
}

const Committees = ({ committees }) => {
  return committees.map(c => (
    <dl>
      <dt>{c.index}</dt>
      <dd className="flex flex-wrap"><Validators validators={c.validators} /></dd>
    </dl>
  ));
}

export default ({ block, committees }) => {
  const [selectedTab, setSelectedTab] = useTabs([
    'overview',
    'committees'
  ]);

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
          </nav>

          <TabPanel hidden={selectedTab !== 'overview'}>
            <dl>
              <dt>Epoch</dt>
              <dd>{block.epoch}</dd>
              <dt>Slot</dt>
              <dd>{block.slot}</dd>
            </dl>
          </TabPanel>
          <TabPanel hidden={selectedTab !== 'committees'}>
            Committees
            <Committees committees={committees} />
          </TabPanel>
        </div>
      </section>
    </>
  )

}