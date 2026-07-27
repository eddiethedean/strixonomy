import { useEffect } from "react";
import { Panel } from "../components/ui";
import { useWorkspaceHost } from "../context/HostContext";

export function SmokePanel(_props?: import("../workspaces/types").WorkspaceProps): JSX.Element {
  const host = useWorkspaceHost();

  useEffect(() => {
    host.postToCore({ type: "ready", panel: "smoke" });
  }, [host]);

  return (
    <Panel>
      <div className="oc-brand">
        <div className="oc-brand-mark" aria-hidden="true" />
        <h1>Strixonomy React</h1>
        <p className="oc-muted">Webview foundation is active.</p>
      </div>
    </Panel>
  );
}
