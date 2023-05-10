import React, { ReactNode, Suspense } from 'react';
import { useRouter } from 'next/router'
import * as Tabs from '@radix-ui/react-tabs';
import cx from 'classnames';
import * as Breadcrumb from "../../components/breadcrumb";
import { useQuery } from '@tanstack/react-query';
import { App, AttestationView, BlockPaths, VoteView, getAttestations, getBlockExtended, getBlockPaths, getCommittees, getVotes, BlockExtendedView, CommitteeView } from '../../pkg/web';
import Link from 'next/link';
import AggregationBits from '../../components/aggregation-bits';
import { Calendar, CaretDown, CaretUp, Certificate, ClockCountdown, Cube, IdentificationBadge, ListChecks, User } from '@phosphor-icons/react';
import { useBuffer } from '../../hooks/data';
import { HighlightCard, BasicCard } from '../../components/card';
import RelativeDatetime from '../../components/relative-datetime';
import Datetime from '../../components/datetime';
import * as RadixSeparator from '@radix-ui/react-separator';
import { Accent, AccentContext } from '../../hooks/accent';
import * as Table from '../../components/table';
import * as Collapsible from '@radix-ui/react-collapsible';

const Separator = ({ className }: { className?: string }) => {
  return <RadixSeparator.Root className={cx(className, "h-1 bg-gradient-to-b from-white to-indigo-50")} />
}

const Validators = ({ validators, aggregationBits }: { validators: number[], aggregationBits?: boolean[] }) => {

  if (!validators) {
    return (
      <p>Loading...</p>
    );
  }

  let validatorsElements = validators.reduce(
    (previousValue: ReactNode[], v: number, i: number) => {
      let el = <span key={i} className={cx({ "text-gray-400": aggregationBits && aggregationBits.length > 0 && !aggregationBits[i] })}>
        {v}
      </span>;

      if (previousValue.length == 0) {
        return [el]
      }
      else {
        return [...previousValue, <>, {el}</>]
      }
    },
    []
  );

  return <Collapsible.Root className=" relative data-closed:truncate pr-6">
    {validatorsElements}
    <div className="absolute inset-y-0 -right-1 p-1">
      <Collapsible.Trigger className="data-open:hidden"><CaretDown /></Collapsible.Trigger>
      <Collapsible.Trigger className="data-closed:hidden"><CaretUp /></Collapsible.Trigger>
    </div>
  </Collapsible.Root>


  return <>{validatorsElements}</>
}

type ModelsProps = { slot: bigint, path: string };

const Committees = ({ slot, path }: ModelsProps) => {
  const { data: committees } = useQuery({
    queryKey: [path],
    queryFn: () => useBuffer(slot, path).then(committeesBuffer => getCommittees(committeesBuffer.buffer)),
    suspense: true
  });

  return <Table.Root>
    <thead>
      <tr>
        <Table.Header className="w-1/6">Index</Table.Header>
        <Table.Header className="w-5/6">Validators</Table.Header>
      </tr>
    </thead>
    <tbody>
      {
        committees.map(
          (c, index) => (
            <tr key={index} >
              <Table.Cell className="text-left">{c.index}</Table.Cell>
              <Table.Cell><Validators validators={c.validators} /></Table.Cell>
            </tr>
          )
        )
        }
    </tbody>
  </Table.Root>
}

const Votes = ({ slot, path }: ModelsProps) => {
  const { data: votes } = useQuery({
    queryKey: [path],
    queryFn: () => useBuffer(slot, path).then(votesBuffer => getVotes(votesBuffer.buffer)),
    suspense: true
  });

  return (
    <div className="flex flex-col gap-2">
      {
        votes.reduce((previousValue: ReactNode[], v: VoteView, i: number) => {
          if (previousValue.length == 0) {
            return [<Vote key={i} vote={v} />]
          }
          else {
            return [...previousValue, <>
              <Separator />
              <Vote key={i} vote={v} />
            </>]
          }

        }, [])
      }
    </div>
  )
}

const Vote = ({ vote }: { vote: VoteView }) => {
  return (
    <div className="grid grid-cols-5 gap-2">
      <div>
        <BasicCard
          className="block-tertiary-card h-24"
          contentClassName="text-5xl"
          title="Slot">
          {vote.slot}
        </BasicCard>
        <BasicCard
          className="block-tertiary-card h-24"
          contentClassName="text-5xl"
          title="Committee index">
          {vote.committeeIndex}
        </BasicCard>
        <BasicCard
          className="block-tertiary-card h-24"
          contentClassName="text-5xl"
          title="Included in block">
          {vote.includedIn}
        </BasicCard>
      </div>
      <div className="col-span-4">
        <BasicCard
          className="block-tertiary-card col-span-4 h-72"
          contentClassName="text-lg flex flex-wrap"
          title="Validators">
          <Validators validators={vote.validators} />
        </BasicCard>
      </div>
    </div>
  );
}

type AttestationsProps = { slot: bigint, paths: BlockPaths };

const Attestations = ({ slot, paths }: AttestationsProps) => {
  const { data: attestations } = useQuery({
    queryKey: [paths.attestations],
    queryFn: () => useBuffer(slot, paths.attestations).then(attestationsBuffer => getAttestations(attestationsBuffer.buffer)),
    suspense: true
  });

  return <>
    {
      attestations.map((a, i) => (
        <div key={i}>
          <h3>Attestation {i}</h3>
          <Attestation slot={slot} attestation={a} committeesPath={paths.committees} />
        </div>
      ))
    }
  </>
}

type AttestationProps = { slot: bigint, attestation: AttestationView, committeesPath: string };

const Attestation = ({ slot, attestation, committeesPath }: AttestationProps) => {
  const { data: committees } = useQuery({
    queryKey: [committeesPath],
    queryFn: () => useBuffer(slot, committeesPath).then(committeesBuffer => getCommittees(committeesBuffer.buffer)),
    suspense: true
  });


  return (
    <dl>
      <dt>Slot</dt>
      <dd>{attestation.slot}</dd>

      <dt>Committee index</dt>
      <dd>{attestation.committeeIndex}</dd>

      <dt>Aggregation bits</dt>
      <dd className="flex flex-wrap">
        <AggregationBits bits={attestation.aggregationBits} />
      </dd>

      <dt>Validators</dt>
      <dd className="flex flex-wrap">
        <Validators
          validators={committees[attestation.committeeIndex].validators}
          aggregationBits={attestation.aggregationBits} />
      </dd>

      <dt>Beacon block root</dt>
      <dd><span className="font-mono">{attestation.beaconBlockRoot}</span></dd>

      <dt>Source</dt>
      <dd>
        Epoch <Link href={`/epoch/${attestation.sourceEpoch}`}>{attestation.sourceEpoch}</Link>
        &nbsp;(<span className="font-mono">{attestation.sourceRoot}</span>)
      </dd>

      <dt>Target</dt>
      <dd>
        Epoch <Link href={`/epoch/${attestation.targetEpoch}`}>{attestation.targetEpoch}</Link>
        &nbsp;(<span className="font-mono">{attestation.targetRoot}</span>)
      </dd>

      <dt>Signature</dt>
      <dd><span className="font-mono break-words">{attestation.signature}</span></dd>
    </dl>
  );
}

const Tab = ({ className, value, children }: { className?: string, value: string, children: ReactNode }) => {
  return (
    <Tabs.Trigger value={value} asChild={true}>
      <span className={cx(className, "text-lg px-1 text-indigo-500 data-active:bg-indigo-500 data-active:text-white data-active:rounded data-inactive:cursor-pointer")}>{children}</span>
    </Tabs.Trigger>
  )
}

const Block = ({ slot }: { slot: bigint }) => {
  const app = new App(process.env.NEXT_PUBLIC_HOST);
  const blockPaths = getBlockPaths(app, slot);

  const { data: block } = useQuery({
    queryKey: [blockPaths.block, blockPaths.blockExtended],
    queryFn: () => {
      return Promise.all([useBuffer(slot, blockPaths.block), useBuffer(slot, blockPaths.blockExtended)])
        .then(([epochBuffer, epochExtendedBuffer]) =>
          getBlockExtended(epochBuffer.buffer, epochExtendedBuffer.buffer, slot)
        )
    },
    suspense: true
  });

  return (
    <Tabs.Root defaultValue="overview" asChild={true}>
      <section>
        <Tabs.List asChild={true}>
          <div className="flex gap-4 my-4">
            <Tab className="font-semibold" value="overview">Overview</Tab>
            <Tab className="font-semibold" value="committees">Committees</Tab>
            <Tab value="votes"><span className="font-semibold">Votes</span> ({block.votesCount})</Tab>
            <Tab value="attestations"><span className="font-semibold">Attestations</span> ({block.attestationsCount})</Tab>
          </div>
        </Tabs.List>
        <Tabs.Content value="overview">
          <Overview block={block} />
        </Tabs.Content>
        <Tabs.Content value="committees">
          <Suspense fallback={<Loading />}>
            <Committees slot={slot} path={blockPaths.committees} />
          </Suspense>
        </Tabs.Content>
        <Tabs.Content value="votes">
          <Suspense fallback={<Loading />}>
            <Votes slot={slot} path={blockPaths.votes} />
          </Suspense>
        </Tabs.Content>
        <Tabs.Content value="attestations">
          <Suspense fallback={<Loading />}>
            <Attestations slot={slot} paths={blockPaths} />
          </Suspense>
        </Tabs.Content>
      </section>
    </Tabs.Root>

  )
}

const Overview = ({ block }: { block: BlockExtendedView }) => {
  return (
    <>
      <div className="grid grid-flow-row grid-cols-5 gap-2">
        <HighlightCard
          className="block-primary-card"
          title="Slot"
          icon={<Cube />}>
          <span className="text-5xl font-semibold">{block.slot}</span>
        </HighlightCard>
        <HighlightCard
          className="epoch-primary-card"
          title="Epoch"
          icon={<ClockCountdown />}>
          <span className="text-5xl font-semibold">{block.epoch}</span>
        </HighlightCard>
        <HighlightCard
          className="bg-gradient-to-b from-green-400 to-green-500"
          title="State"
          icon={<Certificate />}>
          <span className="text-4xl">
            Finalized
          </span>
        </HighlightCard>
        <HighlightCard
          className="validator-primary-card"
          title="Proposer"
          icon={<User />}>
          <span className="text-5xl font-semibold">{block.proposer}</span>
        </HighlightCard>
        <BasicCard
          className="block-secondary-card"
          title="Time"
          icon={<Calendar className="opacity-50" />}>
          <div className="text-3xl">
            <RelativeDatetime timestamp={block.timestamp} /> ago
          </div>
          <div className="text-lg opacity-75">
            <Datetime timestamp={block.timestamp} />
          </div>
        </BasicCard>
        <BasicCard
          className="block-secondary-card"
          title="Attestations"
          icon={<ListChecks className="opacity-50" />}>
          <div className="text-5xl">
            {block.attestationsCount}
          </div>
        </BasicCard>
        <BasicCard
          className="block-secondary-card"
          title="Votes"
          icon={<IdentificationBadge className="opacity-50" />}>
          <div className="text-5xl">
            {block.votesCount}
          </div>
        </BasicCard>
      </div>
      <Separator className="my-2" />
      <div className="flex flex-col gap-2">
        <BasicCard
          className="block-tertiary-card"
          contentClassName="text-2xl"
          title="Block root">
          {block.blockRoot}
        </BasicCard>
        <Separator />
        <BasicCard
          className="block-tertiary-card"
          contentClassName="text-2xl"
          title="Parent root">
          {block.parentRoot}
        </BasicCard>
        <Separator />
        <BasicCard
          className="block-tertiary-card"
          contentClassName="text-2xl"
          title="State root">
          {block.stateRoot}
        </BasicCard>
        <Separator />
        <BasicCard
          className="block-tertiary-card"
          title="Signature">
          <div className="text-xl break-words mr-24">
            {block.signature}
          </div>
        </BasicCard>
        <Separator />
        <BasicCard
          className="block-tertiary-card"
          title="RANDAO Reveal">
          <div className="text-xl break-words mr-24">
            {block.randaoReveal}
          </div>
        </BasicCard>
      </div>
    </>
  )
}

export default () => {

  const router = useRouter();
  const slot = router.query.slot as string;

  return (
    <AccentContext.Provider value={Accent.Indigo}>
      <Breadcrumb.Root>
        <Breadcrumb.Link href="/blocks">
          <Cube />&nbsp;Blocks
        </Breadcrumb.Link>
        <Breadcrumb.Text>{slot}</Breadcrumb.Text>
      </Breadcrumb.Root>
      {slot ? (
        <Suspense fallback={<Loading />}>
          <Block slot={BigInt(slot)} />
        </Suspense>
      ) : (
        <Loading />
      )}
    </AccentContext.Provider>
  )
}

const Loading = () => {
  return (<p>Loading...</p>)
}