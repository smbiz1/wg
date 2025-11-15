import { For, Show } from 'solid-js';
import './ServerSelector.css';

interface Server {
  id: string;
  name: string;
  region: string;
  country: string;
  latency?: number;
}

interface ServerSelectorProps {
  servers: Server[];
  selectedId: string | null;
  onSelect: (id: string) => void;
}

export default function ServerSelector(props: ServerSelectorProps) {
  return (
    <div class="server-selector">
      <div class="selector-header">
        <h3>Select Server Location</h3>
        <Show when={props.servers.length > 0}>
          <p class="selector-hint">
            <Show
              when={props.servers[0]?.latency}
              fallback="Select your preferred location"
            >
              Sorted by speed (fastest first)
            </Show>
          </p>
        </Show>
      </div>

      <div class="server-list">
        <Show
          when={props.servers.length > 0}
          fallback={
            <div class="empty-state">
              <p>No servers available</p>
            </div>
          }
        >
          <For each={props.servers}>
            {(server, index) => (
              <button
                class={`server-item ${props.selectedId === server.id ? 'selected' : ''}`}
                onClick={() => props.onSelect(server.id)}
              >
                <div class="server-info">
                  <div class="server-name">
                    <Show when={index() === 0 && server.latency}>
                      <span class="badge-best">Fastest</span>
                    </Show>
                    {server.name}
                  </div>
                  <div class="server-region">{server.region}</div>
                </div>

                <div class="server-meta">
                  <Show when={server.latency}>
                    <span class="latency">{server.latency}ms</span>
                  </Show>
                  <Show when={props.selectedId === server.id}>
                    <span class="check-icon">✓</span>
                  </Show>
                </div>
              </button>
            )}
          </For>
        </Show>
      </div>
    </div>
  );
}
