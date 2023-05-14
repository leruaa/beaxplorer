import React, { ReactNode, Suspense } from 'react';
import { useRouter } from 'next/router'
import * as Tabs from '@radix-ui/react-tabs';
import cx from 'classnames';
import * as Breadcrumb from "../../components/breadcrumb";
import { useQuery } from '@tanstack/react-query';
import { App, BlockPaths, getAttestations, getBlockExtended, getBlockPaths, getCommittees, getVotes, BlockExtendedView } from '../../pkg/web';
import Link from 'next/link';
import AggregationBits from '../../components/aggregation-bits';
import { Calendar, Certificate, ClockCountdown, Cube, DotsThreeCircle, IconContext, IdentificationBadge, ListChecks, User } from '@phosphor-icons/react';
import { useBuffer } from '../../hooks/data';
import { HighlightCard, BasicCard } from '../../components/card';
import RelativeDatetime from '../../components/relative-datetime';
import Datetime from '../../components/datetime';
import * as RadixSeparator from '@radix-ui/react-separator';
import { Accent, AccentContext } from '../../hooks/accent';
import * as Table from '../../components/table';
import * as RadixTooltip from "@radix-ui/react-tooltip";
import Root from '../../components/root';
import Trim from '../../components/trim';

const Separator = ({ className }: { className?: string }) => {
  return <RadixSeparator.Root className={cx(className, "h-1 bg-gradient-to-b from-white to-indigo-50")} />
}

const Tooltip = ({ title, children }: { title?: string, children: ReactNode }) => {
  return <RadixTooltip.Provider delayDuration={100}>
    <RadixTooltip.Root>
      <RadixTooltip.Trigger><DotsThreeCircle /></RadixTooltip.Trigger>
      <RadixTooltip.Portal>
        <RadixTooltip.Content className="mx-2 p-2 bg-white rounded shadow">
          <RadixTooltip.Arrow className="fill-white" />
          {title && (
            <h4 className="text-xs uppercase font-bold text-indigo-400">{title}</h4>
          )}
          {children}
        </RadixTooltip.Content>
      </RadixTooltip.Portal>
    </RadixTooltip.Root>
  </RadixTooltip.Provider>;
}

const Validators = ({ validators, aggregationBits }: { validators: number[], aggregationBits?: boolean[] }) => {

  if (!validators) {
    return (
      <p>Loading...</p>
    );
  }

  let validatorsElements = validators.map(
    (v, i) => (
      <span key={v} className={cx({ 'w-16': true, "text-gray-400": aggregationBits && aggregationBits.length > 0 && !aggregationBits[i] })}>
        {v}
      </span>
    )
  );

  return <div className="flex flex-wrap">{validatorsElements}</div>
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
        <Table.Header className="w-1/12">Index</Table.Header>
        <Table.Header>Validators</Table.Header>
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

  return <Table.Root>
    <thead>
      <tr>
        <Table.Header>Slot</Table.Header>
        <Table.Header>Committee index</Table.Header>
        <Table.Header>Included in block</Table.Header>
        <Table.Header>Validators</Table.Header>
      </tr>
    </thead>
    <tbody>
      {
        votes.map(
          (v, index) => (
            <tr key={index} >
              <Table.Cell className="text-left">{v.slot}</Table.Cell>
              <Table.Cell className="text-left">{v.committeeIndex}</Table.Cell>
              <Table.Cell className="text-left">{v.includedIn}</Table.Cell>
              <Table.Cell><Validators validators={v.validators} /></Table.Cell>
            </tr>
          )
        )
      }
    </tbody>
  </Table.Root>
}

type AttestationsProps = { slot: bigint, paths: BlockPaths };

const Attestations = ({ slot, paths }: AttestationsProps) => {
  const { data: attestations } = useQuery({
    queryKey: [paths.attestations],
    queryFn: () => useBuffer(slot, paths.attestations).then(attestationsBuffer => getAttestations(attestationsBuffer.buffer)),
    suspense: true
  });

  return <IconContext.Provider
    value={{
      size: "1.25em",
      className: "inline text-indigo-500"
    }}>
    <Table.Root>
      <thead>
        <tr>
          <Table.Header>Index</Table.Header>
          <Table.Header>Slot</Table.Header>
          <Table.Header>Committee index</Table.Header>
          <Table.RightAlignedHeader>Aggregation bits</Table.RightAlignedHeader>
          <Table.RightAlignedHeader>Participation</Table.RightAlignedHeader>
          <Table.RightAlignedHeader>Block root</Table.RightAlignedHeader>
          <Table.RightAlignedHeader>Source epoch</Table.RightAlignedHeader>
          <Table.RightAlignedHeader>Target epoch</Table.RightAlignedHeader>
          <Table.RightAlignedHeader>Signature</Table.RightAlignedHeader>
        </tr>
      </thead>
      <tbody>
        {
          attestations.map(
            (a, index) => (
              <tr key={index}>
                <Table.Cell>{index}</Table.Cell>
                <Table.Cell>{a.slot}</Table.Cell>
                <Table.Cell>{a.committeeIndex}</Table.Cell>
                <Table.RightAlignedCell>
                  <span className="font-mono">{a.aggregationBits.reduce((str, b, i) => str + (i < 8 ? (b ? "1" : "0") : ""), "")}</span>
                  &hellip;&nbsp;
                  <Tooltip title="Aggregation bits">
                    <AggregationBits bits={a.aggregationBits} />
                  </Tooltip>
                </Table.RightAlignedCell>
                <Table.RightAlignedCell>{a.aggregationBits.reduce((sum, b) => sum + (b ? 1 : 0), 0)} / {a.aggregationBits.length}</Table.RightAlignedCell>
                <Table.RightAlignedCell>
                  <Root value={a.beaconBlockRoot} />
                </Table.RightAlignedCell>
                <Table.RightAlignedCell className="whitespace-nowrap">{a.sourceEpoch} (<Root value={a.sourceRoot} />)</Table.RightAlignedCell>
                <Table.RightAlignedCell className="whitespace-nowrap">{a.targetEpoch} (<Root value={a.targetRoot} />)</Table.RightAlignedCell>
                <Table.RightAlignedCell>
                  <Trim value={a.signature} className="font-mono" regEx={/^(.{10}).*$/g} groups={"$1"} />&hellip;
                </Table.RightAlignedCell>
              </tr>
            )
          )
        }
      </tbody>
    </Table.Root>
  </IconContext.Provider>
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