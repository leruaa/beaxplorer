import { useMemo, useCallback, useState } from "react";
import { useRouter } from 'next/router'
import DataTable from "../components/data-table";
import Breadcrumb from "../components/breadcrumb";


export async function getServerSideProps(context) {
  const pageIndex = parseInt(context.query.page, 10) - 1;
  return {
    props: {
      epochs: await getEpochs(pageIndex || 0, 10),
      pageIndex
    }
  }
}

async function getEpochs(pageIndex, pageCount) {
  const wasmModule = await import('../pkg');

  return await wasmModule.get_epochs("http://localhost:3000", pageIndex, pageCount)
}

export default (props) => {

  const router = useRouter()
  const { page } = router.query

  console.log("EPOCHS: " + page);

  const columns = [
    {
      accessor: "epoch",
      Header: "Epoch",
      Cell: ({ value }) => <a href={`/epoch/${value}`}>{value}</a>
    },
    {
      accessor: "ago",
      Header: "Time"
    },
    {
      accessor: "attestations_count",
      Header: "Attestations"
    },
    {
      accessor: "deposits_count",
      Header: "Deposits",
    },
    {
      accessor: "attester_slashings_count",
      Header: "Slashings P / A",
    },
    {
      accessor: "finalized",
      Header: "Finalized"
    },
    {
      accessor: "eligible_ether",
      Header: "Eligible"
    },
    {
      accessor: "voted_ether",
      Header: "Voted"
    },
    {
      accessor: "global_participation_rate",
      Header: "Rate"
    }
  ];

  const [pageCount, setPageCount] = useState(100);
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(true);
  const fetchData = useCallback(async ({ pageSize, pageIndex }) => {
    if (pageIndex == props.pageIndex) {
      setData(props.epochs);
    }
    else {
      setData(await getEpochs(pageIndex, pageSize));
    }
  }, []);

  return (
    <>
      <Breadcrumb breadcrumb={{ parts: [{ text: "Epochs", icon: "clock" }] }} />
      <section className="container mx-auto">
        <div className="tabular-data">
          <DataTable 
            columns={useMemo(() => columns, [])}
            data={data}
            fetchData={fetchData}
            loading={loading}
            pageIndex={page ? parseInt(page as string, 10) - 1 : 0}
            pageCount={pageCount}
          />
        </div>
      </section>
    </>
  )
}
  