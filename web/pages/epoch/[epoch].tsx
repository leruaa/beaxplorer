import { useQuery } from '@tanstack/react-query';
import { useRouter } from 'next/router'
import * as Breadcrumb from "../../components/breadcrumb";
import Percentage from "../../components/percentage";
import Ethers from "../../components/ethers";
import { App, getEpochExtended, getEpochExtendedPath, getEpochPath } from '../../pkg/web';
import RelativeDatetime from '../../components/relative-datetime';
import BlocksTable from '../../components/blocks/blocks-table';
import { useBuffer } from '../../hooks/data';
import { Suspense } from 'react';
import { ClockCountdown } from '@phosphor-icons/react';
import Link from 'next/link';


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
  const epochPath = getEpochPath(app, id);
  const epochExtendedPath = getEpochExtendedPath(app, id);

  const { data: epoch } = useQuery({
    queryKey: [epochPath, epochExtendedPath],
    queryFn: () => {
      return Promise.all([useBuffer(id, epochPath), useBuffer(id, epochExtendedPath)])
        .then(([epochBuffer, epochExtendedBuffer]) =>
          getEpochExtended(epochBuffer.buffer, epochExtendedBuffer.buffer, BigInt(id))
        )
    },
    suspense: true
  });

  return (
    <section className="container mx-auto">
      <div className="tabular-data">
        <p>Showing epoch</p>

        <dl>
          <dt>Epoch</dt>
          <dd>{epoch.epoch}</dd>
          <dt>Finalized</dt>
          <dd>
            {epoch.finalized
              ? <span>Yes</span>
              : <span>No</span>
            }
          </dd>
          <dt>Time</dt>
          <dd>
            <RelativeDatetime timestamp={epoch.timestamp} />
          </dd>
          <dt>Attestations</dt>
          <dd>{epoch.attestationsCount}</dd>
          <dt>Deposits</dt>
          <dd>{epoch.depositsCount}</dd>
          <dt>Slashings P / A</dt>
          <dd>{epoch.proposerSlashingsCount} / {epoch.attesterSlashingsCount}</dd>
          <dt>Voting Participation</dt>
          <dd><Ethers value={epoch.votedEther} /> of <Ethers value={epoch.eligibleEther} /> (<Percentage value={epoch.globalParticipationRate} />)</dd>
        </dl>
      </div>
      <div className="tabular-data">
        <BlocksTable app={app} blocksCount={32} kind={{ kind: "epoch", number: epoch.epoch }} />
      </div>
    </section>
  )
}

const Loading = () => {
  return (<p>"Loading..."</p>)
}