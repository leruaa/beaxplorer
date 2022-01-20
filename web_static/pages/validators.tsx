import { useMemo, useCallback, useState } from "react";
import { useRouter } from 'next/router'
import moment from "moment";
import Moment from 'react-moment';
import DataTable from "../components/data-table";
import Number from "../components/number";
import Ethers from "../components/ethers";
import Trim from "../components/trim";
import Breadcrumb from "../components/breadcrumb";
import { Validators } from "../pkg";


export async function getServerSideProps(context) {
  const validators = await Validators.build("http://localhost:3000");
  const pageIndex = parseInt(context.query.page, 10) - 1;
  return {
    props: {
      validators: await validators.page(pageIndex || 0, 10, "default", false),
      pageIndex
    }
  };
}

export default (props) => {

  const router = useRouter()
  const { page } = router.query
  const validatorsMemo = useMemo(async () => await Validators.build("http://localhost:3000"), []);

  const columns = [
    {
      accessor: "pubkey_hex",
      Header: "Public key",
      Cell: ({ value }) => <><Trim value={value} regEx={/^(.{10}).*$/g} groups={"$1"} />&hellip;</>
    },
    {
      accessor: "validator_index",
      Header: "Index",
      Cell: ({ value }) => <a href={`/validator/${value}`}><Number value={value} /></a>
    },
    {
      accessor: "balance",
      Header: "Balance",
      Cell: ({ value }) => <Ethers value={value} />
    },
    {
      accessor: "activation_epoch",
      Header: "Activation"
    },
    {
      accessor: "exit_epoch",
      Header: "Exit"
    },
    {
      accessor: "withdrawable_epoch",
      Header: "Withdrawable"
    }
  ];

  const getValidatorsCount = useMemo(
    async (): Promise<number> => {
      const validators = await validatorsMemo;
      const meta = await validators.meta();
      return meta.count;
    },
    [],
  );

  const [pageCount, setPageCount] = useState(100);
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(true);
  const fetchData = useCallback(async ({ pageSize, pageIndex, sortBy }) => {
    if (pageIndex == props.pageIndex) {
      setData(props.validators);
    }
    else {
      const validators = await validatorsMemo;
      let sortId = sortBy.length > 0 ? sortBy[0].id : "validator_index";
      let sortDesc = sortBy.length > 0 ? sortBy[0].desc : false;

      if (["validator_index", "activation_epoch"].indexOf(sortId) > -1) sortId = "default";

      setData(
        await validators.page(
          pageIndex,
          pageSize,
          sortId,
          sortDesc
        )
      );
      setPageCount(Math.ceil(await getValidatorsCount / pageSize));
    }
  }, []);

  return (
    <>
      <Breadcrumb breadcrumb={{ parts: [{ text: "Validators", icon: "cube" }] }} />
      <section className="container mx-auto">
        <div className="tabular-data">
          <DataTable
            columns={useMemo(() => columns, [])}
            data={data}
            fetchData={fetchData}
            loading={loading}
            pageIndex={page ? parseInt(page as string, 10) - 1 : 0}
            pageCount={pageCount}
            sortBy={useMemo(() => [{ id: "validator_index", desc: false }], [])}
          />
        </div>
      </section>
    </>
  )
}
