import { useMemo, useCallback, useState } from "react";
import { useRouter } from 'next/router'
import DataTable from "../components/data-table";
import Breadcrumb from "../components/breadcrumb";
import { Epochs } from "../pkg";


export async function getServerSideProps(context) {
  const epochs = await Epochs.build("http://localhost:3000");
  const pageIndex = parseInt(context.query.page, 10) - 1;
  return {
    props: {
      epochs: await epochs.page(pageIndex || 0, 10),
      pageIndex
    }
  }
}

export default (props) => {

  const router = useRouter()
  const { page } = router.query
  const epochsMemo = useMemo(async () => await Epochs.build("http://localhost:3000"), []);

  const columns = [
    {
      accessor: "epoch",
      Header: "Epoch",
      Cell: ({ value }) => <a href={`/epoch/${value}`}>{value}</a>
    },
    {
      accessor: "timestamp",
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

  const getEpochsCount = useMemo(
    async (): Promise<number> => {
      const epochs = await epochsMemo;
      const meta = await epochs.meta();
      return meta.count;
    },
    [],
  );

  const [pageCount, setPageCount] = useState(100);
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(true);
  const fetchData = useCallback(async ({ pageSize, pageIndex }) => {
    if (pageIndex == props.pageIndex) {
      setData(props.epochs);
    }
    else {
      const epochs = await epochsMemo;
      setData(await epochs.page(pageIndex, pageSize));
      setPageCount(Math.ceil(await getEpochsCount / pageSize));
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
  