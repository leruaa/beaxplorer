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
import Bdg from '../../components/badge';
import Card from '../../components/card';
import { Calendar, Certificate, ClockCountdown, IdentificationBadge, ListChecks, SealCheck, SealWarning } from '@phosphor-icons/react';
import Link from 'next/link';
import Datetime from '../../components/datetime';
import { Root as Separator } from '@radix-ui/react-separator';


export default (props) => {
  const router = useRouter();
  const id = router.query.epoch as string;

  return (
    <>
      <Breadcrumb.Root>
        <Breadcrumb.Part>
          <Link href="/epochs"><ClockCountdown />&nbsp;Epochs</Link>
        </Breadcrumb.Part>
        <Breadcrumb.Part>
          <span>{id}</span>
        </Breadcrumb.Part>
      </Breadcrumb.Root>
      {id ? (
        <Suspense fallback={<Loading />}>
          <Epoch id={BigInt(id)} />
        </Suspense>
      ) : (
        <Loading />
      )}
    </>
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
    <section className="container mx-auto">
      <div className="grid grid-flow-row grid-cols-5 gap-2">
        <Card
          className="epoch-primary-card"
          titleClassName="opacity-70"
          title="Epoch"
          icon={<ClockCountdown />}>
          <span className="text-5xl font-semibold">{epoch.epoch}</span>
        </Card>
        <Card
          className="bg-gradient-to-b from-green-400 to-green-500"
          titleClassName="opacity-70"
          title="State"
          icon={<Certificate />}>
          <span className="text-4xl">
            Finalized
          </span>
        </Card>
        <Card
          className="epoch-secondary-card"
          titleClassName="opacity-50"
          title="Time"
          icon={<Calendar className="opacity-50" />}>
          <div className="text-3xl">
            <RelativeDatetime timestamp={epoch.timestamp} /> ago
          </div>
          <div className="text-lg opacity-75">
            <Datetime timestamp={epoch.timestamp} />
          </div>
        </Card>
        <Card
          className="bg-gradient-to-b from-green-400 to-green-500"
          titleClassName="opacity-70"
          title="Proposed blocks count"
          icon={<SealCheck />}>
          <span className="text-5xl font-semibold">
            {epoch.proposedBlocksCount}
          </span>
        </Card>
        <Card
          className="bg-gradient-to-b from-yellow-400 to-yellow-500"
          titleClassName="opacity-70"
          title="Missed blocks count"
          icon={<SealWarning />}>
          <span className="text-5xl font-semibold">
            {epoch.missedBlocksCount}
          </span>
        </Card>
        <Card
          className="epoch-secondary-card"
          titleClassName="opacity-50"
          title="Attestations"
          icon={<ListChecks className="opacity-50" />}>
          <div className="text-5xl font-semibold">
            {epoch.attestationsCount}
          </div>
        </Card>
        <Card
          className="col-span-2 epoch-secondary-card"
          titleClassName="opacity-50"
          title="Voting participation"
          icon={<IdentificationBadge className="opacity-50" />}>
          <div className="text-3xl font-semibold">
            <Ethers value={epoch.votedEther} /> of <Ethers value={epoch.eligibleEther} />&nbsp;ETH{" "}
            (<Percentage value={epoch.globalParticipationRate} />)
          </div>
        </Card>
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