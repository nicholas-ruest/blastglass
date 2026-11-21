import { useEffect, useMemo, useState } from "react";
import type { FormEvent } from "react";
import { api, type EvidenceRecord, type RehearsalResponse } from "./api/client";

type View = "studio" | "evidence";
const defaults = {service:"payments-api",scenario:"regional dependency loss",blast_radius:35,recovery_minutes:12};

function EvidenceCard({record}:{record:EvidenceRecord}) {
  const payload = useMemo(()=>{try{return JSON.parse(record.payload) as {service?:string;scenario?:string;risk_score?:number}}catch{return {}}},[record.payload]);
  return <article className="evidence-card" data-testid="evidence-card">
    <div><span className="status-dot"/>sealed</div><h3>{payload.service ?? "Rehearsal evidence"}</h3>
    <p>{payload.scenario ?? "Recorded operational rehearsal"}</p>
    <dl><div><dt>Correlation</dt><dd>{record.correlation_id}</dd></div><div><dt>Risk</dt><dd>{typeof payload.risk_score==="number"?payload.risk_score.toFixed(2):"n/a"}</dd></div></dl>
    <code>{record.hash.slice(0,22)}...</code>
  </article>;
}

export function App() {
  const [view,setView]=useState<View>("studio");
  const [form,setForm]=useState(defaults);
  const [role,setRole]=useState("rehearsal-operator");
  const [result,setResult]=useState<RehearsalResponse|null>(null);
  const [records,setRecords]=useState<EvidenceRecord[]>([]);
  const [loading,setLoading]=useState(false);
  const [auditLoading,setAuditLoading]=useState(true);
  const [error,setError]=useState("");

  async function refreshAudit(){setAuditLoading(true);try{setRecords(await api.audit())}catch(e){setError(e instanceof Error?e.message:"Audit unavailable")}finally{setAuditLoading(false)}}
  useEffect(()=>{void refreshAudit()},[]);
  async function submit(event:FormEvent){event.preventDefault();setLoading(true);setError("");setResult(null);try{const created=await api.create(form,role);setResult(created);await refreshAudit()}catch(e){setError(e instanceof Error?e.message:"Rehearsal failed")}finally{setLoading(false)}}

  return <div className="app-shell">
    <header><a className="brand" href="#main" aria-label="Blastglass home"><span className="brand-mark">BG</span><span>BLASTGLASS<small>RESILIENCE STUDIO</small></span></a>
      <nav aria-label="Primary"><button className={view==="studio"?"active":""} onClick={()=>setView("studio")}>Studio</button><button className={view==="evidence"?"active":""} onClick={()=>setView("evidence")}>Evidence <b>{records.length}</b></button></nav>
      <div className="live"><span/>local / ready</div></header>
    <main id="main">
      {view==="studio"?<>
        <section className="hero"><div><p className="eyebrow">CONTROLLED FAILURE LAB</p><h1>Rehearse the outage.<br/><em>Protect the outcome.</em></h1><p>Model an operational disruption, enforce policy boundaries, and retain tamper-evident proof of the recovery decision.</p></div><div className="mesh" aria-hidden="true"><i/><i/><i/><i/></div></section>
        <section className="studio-grid">
          <form onSubmit={submit} aria-label="Rehearsal composer"><div className="panel-title"><span>01</span><div><h2>Compose rehearsal</h2><p>Define the boundary and recovery objective.</p></div></div>
            <label>Service<input value={form.service} onChange={e=>setForm({...form,service:e.target.value})} required/></label>
            <label>Failure scenario<textarea value={form.scenario} onChange={e=>setForm({...form,scenario:e.target.value})} required/></label>
            <div className="row"><label>Blast radius <output>{form.blast_radius}%</output><input aria-label="Blast radius" type="range" min="1" max="100" value={form.blast_radius} onChange={e=>setForm({...form,blast_radius:Number(e.target.value)})}/></label><label>Recovery target<input aria-label="Recovery minutes" type="number" min="1" max="240" value={form.recovery_minutes} onChange={e=>setForm({...form,recovery_minutes:Number(e.target.value)})}/><span className="unit">MIN</span></label></div>
            <label>Operator role<select value={role} onChange={e=>setRole(e.target.value)}><option value="rehearsal-operator">Rehearsal operator</option><option value="observer">Observer (denied)</option></select></label>
            <button className="launch" disabled={loading}>{loading?<><span className="spinner"/>Running seven integrations...</>:"Run controlled rehearsal"}</button>
          </form>
          <section className="result-panel" aria-live="polite"><div className="panel-title"><span>02</span><div><h2>Decision evidence</h2><p>Correlated results from the execution path.</p></div></div>
            {error&&<div className="error" role="alert"><strong>Boundary held</strong><p>{error}</p><button onClick={()=>{setRole("rehearsal-operator");setError("")}}>Restore operator access</button></div>}
            {!error&&!result&&!loading&&<div className="empty"><div className="radar"/><h3>Awaiting rehearsal</h3><p>Results, upstream receipts, and the evidence seal will appear here.</p></div>}
            {loading&&<div className="empty"><div className="radar running"/><h3>Executing complete path</h3><p>Policy, domain stages, and upstream responsibilities are being correlated.</p></div>}
            {result&&<div className="success" data-testid="result"><div className="score"><span>Composite risk</span><strong>{result.risk_score.toFixed(2)}</strong></div><h3>Rehearsal completed</h3><p>{result.stages.length} accountable stages returned evidence.</p><div className="stage-list">{result.stages.map(stage=><span key={stage}>{stage}</span>)}</div><dl><div><dt>Correlation</dt><dd>{result.correlation_id}</dd></div><div><dt>Evidence seal</dt><dd>{result.evidence_hash.slice(0,26)}...</dd></div></dl></div>}
          </section>
        </section></>:<section className="vault"><p className="eyebrow">DURABLE AUDIT VAULT</p><h1>Every rehearsal leaves a verifiable trail.</h1><button onClick={()=>void refreshAudit()}>Refresh audit</button>{auditLoading?<p className="loading-copy">Loading sealed records...</p>:records.length===0?<div className="empty"><h3>No evidence yet</h3><p>Complete a rehearsal to create the first hash-chained record.</p></div>:<div className="evidence-grid">{records.map(r=><EvidenceCard key={r.id} record={r}/>)}</div>}</section>}
    </main>
    <footer><span>Blastglass / local-first</span><span>7 upstreams · 25 Rust crates · immutable audit</span></footer>
  </div>;
}
