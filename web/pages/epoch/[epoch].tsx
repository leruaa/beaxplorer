import moment from "moment";
import Moment from 'react-moment';
import { useQuery } from '@tanstack/react-query';
import { useRouter } from 'next/router'
import Breadcrumb from "../../components/breadcrumb";
import Percentage from "../../components/percentage";
import Ethers from "../../components/ethers";
import { App, getEpochExtended } from '../../pkg/web';


export async function getServerSideProps(context) {
  const host = process.env.HOST;
  return {
    props: {
      host
    }
  }
}

export default (props) => {
  const app = new App(props.host);
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
                <span title={moment.unix(epoch.timestamp).format("L LTS")}>
                  <Moment unix fromNow date={epoch.timestamp} />
                </span>
              </dd>
              <dt>Attestations</dt>
              <dd>{epoch.attestations_count}</dd>
              <dt>Deposits</dt>
              <dd>{epoch.deposits_count}</dd>
              <dt>Slashings P / A</dt>
              <dd>{epoch.proposer_slashings_count} / {epoch.attester_slashings_count}</dd>
              <dt>Voting Participation</dt>
              <dd><Ethers value={epoch.eligible_ether} /> of <Ethers value={epoch.voted_ether} /> (<Percentage value={epoch.global_participation_rate} />)</dd>
            </dl>
          </div>
        </section>
      }
    </>
  )

}