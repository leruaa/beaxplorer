import { useRouter } from 'next/router'
import Breadcrumb from "../../components/breadcrumb";
import { Epochs } from "../../pkg";

export async function getServerSideProps(context) {
  const wasmModule = await import('../../pkg');
  const epochs = await Epochs.build("http://localhost:3000");
  return {
    props: await epochs.get(context.params.epoch)
  }
}

export default (props) => {
  const router = useRouter()
  const { epoch } = router.query

  return (
    <>
      <Breadcrumb breadcrumb={{ parts: [{ text: "Epochs", icon: "clock" }] }} />
      <section className="container mx-auto">
        <div className="tabular-data">
          <p>Showing epoch</p>

          <dl>
            <dt>Epoch</dt>
            <dd>{props.epoch} ({epoch})</dd>
            <dt>Finalized</dt>
            <dd>
              {props.finalized
                ? <span>Yes</span>
                : <span>No</span>
              }
            </dd>
            <dt>Time</dt>
            <dd>{props.time} ({props.ago})</dd>
            <dt>Attestations</dt>
            <dd>{props.attestations_count}</dd>
            <dt>Deposits</dt>
            <dd>{props.deposits_count}</dd>
            <dt>Slashings P / A</dt>
            <dd>{props.proposer_slashings_count} / {props.attester_slashings_count}</dd>
            <dt>Voting Participation</dt>
            <dd>{props.eligible_ether} ETH of {props.voted_ether} ETH ({props.global_participation_percentage}%)</dd>
          </dl>
        </div>
      </section>
    </>
  )

}