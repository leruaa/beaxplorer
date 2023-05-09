import { useQuery } from '@tanstack/react-query';
import { useRouter } from 'next/router'
import * as Breadcrumb from "../../components/breadcrumb";
import Percentage from "../../components/percentage";
import Ethers from "../../components/ethers";
import { App, getEpochExtended, getEpochPaths } from '../../pkg/web';
import RelativeDatetime from '../../components/relative-datetime';
import BlocksTable from '../../components/blocks/blocks-table';
import { useBuffer } from '../../hooks/data';
import { Suspense } from 'react';
import { HighlightCard, BasicCard } from '../../components/card';
import { Calendar, Certificate, ClockCountdown, IdentificationBadge, ListChecks, SealCheck, SealWarning } from '@phosphor-icons/react';
import Datetime from '../../components/datetime';
import { Root as Separator } from '@radix-ui/react-separator';
import { Accent, AccentContext } from '../../hooks/accent';


export default (props) => {
  const router = useRouter();
  const id = router.query.epoch as string;

  return (
    <AccentContext.Provider value={Accent.Sky}>
      <Breadcrumb.Root>
        <Breadcrumb.Link href="/epochs">
          <ClockCountdown />&nbsp;Epochs
        </Breadcrumb.Link>
        <Breadcrumb.Text>{id}</Breadcrumb.Text>
      </Breadcrumb.Root>
      {id ? (
        <Suspense fallback={<Loading />}>
          <Epoch id={BigInt(id)} />
        </Suspense>
      ) : (
        <Loading />
      )}
    </AccentContext.Provider>
  )
}

const Epoch = ({ id }: { id: bigint }) => {
  const app = new App(process.env.NEXT_PUBLIC_HOST);
  const epochPaths = getEpochPaths(app, id);

  const { data: epoch } = useQuery({
    queryKey: [epochPaths],
    queryFn: () => {
      return Promise.all([useBuffer(id, epochPaths.epoch), useBuffer(id, epochPaths.epochExtended)])
        .then(([epochBuffer, epochExtendedBuffer]) =>
          getEpochExtended(epochBuffer.buffer, epochExtendedBuffer.buffer, id)
        )
    },
    suspense: true
  });

  return (
    <section>
      <div className="grid grid-flow-row grid-cols-5 gap-2">
        <HighlightCard
          className="epoch-primary-card"
          title="Epoch"
          icon={<ClockCountdown />}>
          <span className="text-5xl font-semibold">{epoch.epoch}</span>
        </HighlightCard>
        <HighlightCard
          className="bg-gradient-to-b from-green-400 to-green-500"
          title="State"
          icon={<Certificate />}>
          <span className="text-4xl">
            Finalized
          </span>
        </HighlightCard>
        <BasicCard
          className="epoch-secondary-card"
          title="Time"
          icon={<Calendar className="opacity-50" />}>
          <div className="text-3xl">
            <RelativeDatetime timestamp={epoch.timestamp} /> ago
          </div>
          <div className="text-lg opacity-75">
            <Datetime timestamp={epoch.timestamp} />
          </div>
        </BasicCard>
        <HighlightCard
          className="bg-gradient-to-b from-green-400 to-green-500"
          title="Proposed blocks count"
          icon={<SealCheck />}>
          <span className="text-5xl font-semibold">
            {epoch.proposedBlocksCount}
          </span>
        </HighlightCard>
        <HighlightCard
          className="bg-gradient-to-b from-yellow-400 to-yellow-500"
          title="Missed blocks count"
          icon={<SealWarning />}>
          <span className="text-5xl font-semibold">
            {epoch.missedBlocksCount}
          </span>
        </HighlightCard>
        <BasicCard
          className="epoch-secondary-card"
          title="Attestations"
          icon={<ListChecks className="opacity-50" />}>
          <div className="text-5xl font-semibold">
            {epoch.attestationsCount}
          </div>
        </BasicCard>
        <BasicCard
          className="col-span-2 epoch-secondary-card"
          title="Voting participation"
          icon={<IdentificationBadge className="opacity-50" />}>
          <div className="text-3xl font-semibold">
            <Ethers value={epoch.votedEther} /> of <Ethers value={epoch.eligibleEther} />&nbsp;ETH{" "}
            (<Percentage value={epoch.globalParticipationRate} />)
          </div>
        </BasicCard>
      </div>

      <Separator className="my-5" />

      <div className="tabular-data">
        <BlocksTable app={app} blocksCount={32} kind={{ kind: "epoch", number: epoch.epoch }} />
      </div>
    </section>
  )
}

const Loading = () => {
  return (<p>Loading...</p>)
}