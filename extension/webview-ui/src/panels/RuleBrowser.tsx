import { useEffect, useState } from "react";
import { PanelMain } from "../a11y";
import {
  Callout,
  InlineCode,
  LoadingState,
  Panel,
  PanelHeader,
  Section,
  StickyActions,
  Toolbar,
  ToolbarGroup,
} from "../components/ui";
import { getVsCodeApi } from "../vscodeApi";
import { HostMessage, isHostMessage } from "../messages";

export interface SwrlRuleRow {
  id: string;
  label: string;
  body_count: number;
  head_count: number;
  enabled: boolean;
  rule_json?: string;
  document_uri?: string;
  ontology_iri?: string;
}

const EMPTY_RULE = `{
  "body": [
    { "kind": "class", "class": "http://example.org/swrl#Person", "arg": { "variable": "x" } }
  ],
  "head": [
    { "kind": "class", "class": "http://example.org/swrl#Human", "arg": { "variable": "x" } }
  ],
  "enabled": true
}`;

export function RuleBrowserPanel(): JSX.Element {
  const [rules, setRules] = useState<SwrlRuleRow[] | null>(null);
  const [error, setError] = useState("");

  useEffect(() => {
    getVsCodeApi().postMessage({ type: "ready", panel: "ruleBrowser" });
    getVsCodeApi().postMessage({ type: "refreshSwrlRules" });
    const handler = (event: MessageEvent): void => {
      if (!isHostMessage(event.data)) {
        return;
      }
      const msg: HostMessage = event.data;
      if (msg.type === "swrlRulesLoaded") {
        setRules(msg.rules);
        setError("");
      }
      if (msg.type === "error") {
        setError(msg.message);
      }
    };
    window.addEventListener("message", handler);
    return () => window.removeEventListener("message", handler);
  }, []);

  return (
    <Panel>
      <PanelMain label="SWRL Rule Browser">
      <PanelHeader title="SWRL Rule Browser" subtitle="Workspace swrlRule annotations" />
      {error ? <Callout variant="error">{error}</Callout> : null}
      <Toolbar>
        <ToolbarGroup>
          <button
            type="button"
            onClick={() => getVsCodeApi().postMessage({ type: "refreshSwrlRules" })}
          >
            Refresh
          </button>
          <button
            type="button"
            onClick={() =>
              getVsCodeApi().postMessage({
                type: "openSwrlRuleEditor",
                ruleJson: EMPTY_RULE,
              })
            }
          >
            New rule
          </button>
        </ToolbarGroup>
      </Toolbar>
      {rules === null ? (
        <LoadingState label="Loading SWRL rules…" />
      ) : rules.length === 0 ? (
        <Callout variant="info">No SWRL rules found in open documents or indexed Turtle files.</Callout>
      ) : (
        <Section title={`Rules (${rules.length})`}>
          <ul className="oc-list">
            {rules.map((rule) => (
              <li key={`${rule.document_uri ?? ""}:${rule.id}`} className="oc-list-row">
                <div>
                  <strong>{rule.id}</strong>{" "}
                  <span className={rule.enabled ? "" : "oc-muted"}>({rule.enabled ? "enabled" : "disabled"})</span>
                  <div className="oc-muted">
                    <InlineCode>{rule.label}</InlineCode> · body {rule.body_count} · head{" "}
                    {rule.head_count}
                  </div>
                </div>
                <button
                  type="button"
                  onClick={() =>
                    getVsCodeApi().postMessage({
                      type: "openSwrlRuleEditor",
                      ruleJson: rule.rule_json,
                      documentUri: rule.document_uri,
                      ontologyIri: rule.ontology_iri,
                    })
                  }
                >
                  Edit
                </button>
              </li>
            ))}
          </ul>
        </Section>
      )}
      <StickyActions>
        <button
          type="button"
          onClick={() =>
            getVsCodeApi().postMessage({
              type: "openSwrlRuleEditor",
              ruleJson: EMPTY_RULE,
            })
          }
        >
          New rule
        </button>
      </StickyActions>
      </PanelMain>
    </Panel>
  );
}
