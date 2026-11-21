export type CreateRehearsal = { service:string; scenario:string; blast_radius:number; recovery_minutes:number };
export type RehearsalResponse = { id:string; correlation_id:string; status:string; risk_score:number; stages:string[]; evidence_hash:string };
export type ErrorResponse = { code:string; message:string; correlation_id:string };
export type EvidenceRecord = { id:string; correlation_id:string; payload:string; previous_hash:string; hash:string };

const headers = (role="rehearsal-operator"): HeadersInit => ({
  "content-type":"application/json",
  authorization:"Bearer blastglass-operator",
  "x-operator":"studio@example.com",
  "x-role":role
});

async function decode<T>(response:Response):Promise<T> {
  if (response.ok) return response.json() as Promise<T>;
  const body = await response.text();
  try { const error=JSON.parse(body) as ErrorResponse; throw new Error(error.message); }
  catch (error) { if (error instanceof SyntaxError) throw new Error(body || `HTTP ${response.status}`); throw error; }
}

export const api = {
  create(input:CreateRehearsal, role?:string) {
    return fetch("/api/v1/rehearsals",{method:"POST",headers:headers(role),body:JSON.stringify(input)}).then(decode<RehearsalResponse>);
  },
  audit() { return fetch("/api/v1/audit").then(decode<EvidenceRecord[]>); }
};
