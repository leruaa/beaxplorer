import { useQuery } from '@tanstack/react-query';
import { useRouter } from 'next/router'
import { DateTime } from "luxon";
import Breadcrumb from "../../components/breadcrumb";
import Percentage from "../../components/percentage";
import Ethers from "../../components/ethers";
import { App, getEpochExtended } from '../../pkg/web';
import RelativeDatetime from '../../components/relative-datetime';


export default (props) => {
  const app = new App(process.env.NEXT_PUBLIC_HOST);
  const router = useRouter();
  const id = router.query.epoch as string;
  const { isLoading, error, data: epoch } = useQuery(
    ["epoch", id],
    () => getEpochExtended(app, BigInt(id))
  );

  return (
    <>
      <Breadcrumb breadcrumb={{ parts: [{ text: "Epochs", icon: "clock" }] }} />
      {isLoading ? "Loading..." :
        <section className="container mx-auto">
          <div className="tabular-data">
            <p>Showing epoch</p>

            <dl>
              <dt>Epoch</dt>
              <dd>{epoch.epoch} ({id})</dd>
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
              <dd><Ethers value={epoch.votedEther} /> of <Ethers value={epoch.eligibleEther} /> (<Percentage value={epoch.global_participation_rate} />)</dd>
            </dl>
          </div>
        </section>
      }
    </>
  )

}