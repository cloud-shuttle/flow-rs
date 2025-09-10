# Integration Examples for Leptos Flow Simple Example

This document provides examples of how to integrate the Leptos Flow Simple Example with various frameworks, libraries, and platforms.

## Table of Contents

- [React Integration](#react-integration)
- [Vue.js Integration](#vuejs-integration)
- [Angular Integration](#angular-integration)
- [Vanilla JavaScript](#vanilla-javascript)
- [Node.js Backend Integration](#nodejs-backend-integration)
- [WebSocket Real-time Updates](#websocket-real-time-updates)
- [REST API Integration](#rest-api-integration)
- [Database Integration](#database-integration)
- [Mobile Integration](#mobile-integration)
- [Desktop Integration](#desktop-integration)

## React Integration

### Basic React Component

```tsx
// FlowCanvas.tsx
import React, { useEffect, useRef, useState } from 'react';
import init, { create_simple_flow } from './pkg/simple_flow_example';

interface FlowCanvasProps {
  width?: number;
  height?: number;
  onNodeClick?: (nodeId: string) => void;
  onNodeDrag?: (nodeId: string, x: number, y: number) => void;
}

export const FlowCanvas: React.FC<FlowCanvasProps> = ({
  width = 800,
  height = 600,
  onNodeClick,
  onNodeDrag,
}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [isInitialized, setIsInitialized] = useState(false);

  useEffect(() => {
    const initWasm = async () => {
      await init();
      setIsInitialized(true);
    };
    initWasm();
  }, []);

  useEffect(() => {
    if (isInitialized && canvasRef.current) {
      // Set canvas size
      canvasRef.current.width = width;
      canvasRef.current.height = height;

      // Create flow diagram
      create_simple_flow();

      // Set up event handlers
      if (onNodeClick) {
        canvasRef.current.addEventListener('click', (event) => {
          // Handle node click logic
          const rect = canvasRef.current!.getBoundingClientRect();
          const x = event.clientX - rect.left;
          const y = event.clientY - rect.top;
          // Determine which node was clicked and call onNodeClick
        });
      }
    }
  }, [isInitialized, width, height, onNodeClick, onNodeDrag]);

  return (
    <div className="flow-container">
      <canvas
        ref={canvasRef}
        style={{ border: '1px solid #ccc', borderRadius: '8px' }}
      />
    </div>
  );
};
```

### React Hook for Flow Management

```tsx
// useFlowCanvas.ts
import { useEffect, useRef, useState, useCallback } from 'react';
import init, {
  create_simple_flow,
  add_node_at_position,
  move_node,
  remove_node
} from './pkg/simple_flow_example';

interface FlowNode {
  id: string;
  x: number;
  y: number;
  label: string;
  color?: string;
}

interface FlowEdge {
  id: string;
  source: string;
  target: string;
}

interface UseFlowCanvasOptions {
  width?: number;
  height?: number;
  initialNodes?: FlowNode[];
  initialEdges?: FlowEdge[];
}

export const useFlowCanvas = (options: UseFlowCanvasOptions = {}) => {
  const {
    width = 800,
    height = 600,
    initialNodes = [],
    initialEdges = [],
  } = options;

  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [isInitialized, setIsInitialized] = useState(false);
  const [nodes, setNodes] = useState<FlowNode[]>(initialNodes);
  const [edges, setEdges] = useState<FlowEdge[]>(initialEdges);
  const [selectedNode, setSelectedNode] = useState<string | null>(null);

  // Initialize WASM
  useEffect(() => {
    const initWasm = async () => {
      await init();
      setIsInitialized(true);
    };
    initWasm();
  }, []);

  // Create initial flow
  useEffect(() => {
    if (isInitialized && canvasRef.current) {
      canvasRef.current.width = width;
      canvasRef.current.height = height;
      create_simple_flow();
    }
  }, [isInitialized, width, height]);

  // Add node
  const addNode = useCallback((node: FlowNode) => {
    add_node_at_position(node.x, node.y);
    setNodes(prev => [...prev, node]);
  }, []);

  // Move node
  const moveNode = useCallback((nodeId: string, x: number, y: number) => {
    move_node(nodeId, x, y);
    setNodes(prev => prev.map(node =>
      node.id === nodeId ? { ...node, x, y } : node
    ));
  }, []);

  // Remove node
  const removeNode = useCallback((nodeId: string) => {
    remove_node(nodeId);
    setNodes(prev => prev.filter(node => node.id !== nodeId));
    setEdges(prev => prev.filter(edge =>
      edge.source !== nodeId && edge.target !== nodeId
    ));
  }, []);

  // Handle canvas click
  const handleCanvasClick = useCallback((event: React.MouseEvent<HTMLCanvasElement>) => {
    if (!canvasRef.current) return;

    const rect = canvasRef.current.getBoundingClientRect();
    const x = event.clientX - rect.left;
    const y = event.clientY - rect.top;

    // Check if clicking on existing node or empty space
    const clickedNode = nodes.find(node =>
      Math.abs(node.x - x) < 50 && Math.abs(node.y - y) < 30
    );

    if (clickedNode) {
      setSelectedNode(clickedNode.id);
    } else {
      setSelectedNode(null);
    }
  }, [nodes]);

  return {
    canvasRef,
    isInitialized,
    nodes,
    edges,
    selectedNode,
    addNode,
    moveNode,
    removeNode,
    handleCanvasClick,
  };
};
```

### React Flow Component with Controls

```tsx
// FlowEditor.tsx
import React, { useState } from 'react';
import { FlowCanvas, useFlowCanvas } from './FlowCanvas';

interface FlowEditorProps {
  onSave?: (nodes: FlowNode[], edges: FlowEdge[]) => void;
  onLoad?: () => { nodes: FlowNode[]; edges: FlowEdge[] };
}

export const FlowEditor: React.FC<FlowEditorProps> = ({ onSave, onLoad }) => {
  const [isAddingNode, setIsAddingNode] = useState(false);
  const [nodeColor, setNodeColor] = useState('#3b82f6');

  const {
    canvasRef,
    isInitialized,
    nodes,
    edges,
    selectedNode,
    addNode,
    moveNode,
    removeNode,
    handleCanvasClick,
  } = useFlowCanvas();

  const handleAddNode = () => {
    setIsAddingNode(true);
  };

  const handleCanvasClickWithAdd = (event: React.MouseEvent<HTMLCanvasElement>) => {
    if (isAddingNode) {
      const rect = canvasRef.current!.getBoundingClientRect();
      const x = event.clientX - rect.left;
      const y = event.clientY - rect.top;

      const newNode: FlowNode = {
        id: `node_${Date.now()}`,
        x,
        y,
        label: `Node ${nodes.length + 1}`,
        color: nodeColor,
      };

      addNode(newNode);
      setIsAddingNode(false);
    } else {
      handleCanvasClick(event);
    }
  };

  const handleSave = () => {
    if (onSave) {
      onSave(nodes, edges);
    }
  };

  const handleLoad = () => {
    if (onLoad) {
      const data = onLoad();
      // Load data into flow
    }
  };

  if (!isInitialized) {
    return <div>Loading...</div>;
  }

  return (
    <div className="flow-editor">
      <div className="toolbar">
        <button
          onClick={handleAddNode}
          className={isAddingNode ? 'active' : ''}
        >
          Add Node
        </button>
        <input
          type="color"
          value={nodeColor}
          onChange={(e) => setNodeColor(e.target.value)}
        />
        {selectedNode && (
          <button onClick={() => removeNode(selectedNode)}>
            Delete Node
          </button>
        )}
        <button onClick={handleSave}>Save</button>
        <button onClick={handleLoad}>Load</button>
      </div>

      <FlowCanvas
        ref={canvasRef}
        width={800}
        height={600}
        onCanvasClick={handleCanvasClickWithAdd}
      />

      <div className="properties">
        {selectedNode && (
          <div>
            <h3>Node Properties</h3>
            <p>ID: {selectedNode}</p>
            <p>Position: {nodes.find(n => n.id === selectedNode)?.x}, {nodes.find(n => n.id === selectedNode)?.y}</p>
          </div>
        )}
      </div>
    </div>
  );
};
```

## Vue.js Integration

### Vue 3 Composition API

```vue
<!-- FlowCanvas.vue -->
<template>
  <div class="flow-container">
    <canvas
      ref="canvasRef"
      :width="width"
      :height="height"
      @click="handleCanvasClick"
      @mousedown="handleMouseDown"
      @mousemove="handleMouseMove"
      @mouseup="handleMouseUp"
    />
    <div v-if="!isInitialized" class="loading">
      Loading Flow Canvas...
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import init, {
  create_simple_flow,
  add_node_at_position,
  move_node
} from './pkg/simple_flow_example';

interface Props {
  width?: number;
  height?: number;
  nodes?: FlowNode[];
  edges?: FlowEdge[];
}

interface Emits {
  (e: 'node-click', nodeId: string): void;
  (e: 'node-move', nodeId: string, x: number, y: number): void;
  (e: 'canvas-click', x: number, y: number): void;
}

const props = withDefaults(defineProps<Props>(), {
  width: 800,
  height: 600,
  nodes: () => [],
  edges: () => [],
});

const emit = defineEmits<Emits>();

const canvasRef = ref<HTMLCanvasElement>();
const isInitialized = ref(false);
const isDragging = ref(false);
const dragStart = ref({ x: 0, y: 0 });

// Initialize WASM
onMounted(async () => {
  await init();
  isInitialized.value = true;

  if (canvasRef.value) {
    create_simple_flow();
  }
});

// Watch for node changes
watch(() => props.nodes, (newNodes) => {
  if (isInitialized.value) {
    // Update canvas with new nodes
    newNodes.forEach(node => {
      add_node_at_position(node.x, node.y);
    });
  }
}, { deep: true });

const handleCanvasClick = (event: MouseEvent) => {
  if (!canvasRef.value) return;

  const rect = canvasRef.value.getBoundingClientRect();
  const x = event.clientX - rect.left;
  const y = event.clientY - rect.top;

  // Check if clicking on a node
  const clickedNode = props.nodes.find(node =>
    Math.abs(node.x - x) < 50 && Math.abs(node.y - y) < 30
  );

  if (clickedNode) {
    emit('node-click', clickedNode.id);
  } else {
    emit('canvas-click', x, y);
  }
};

const handleMouseDown = (event: MouseEvent) => {
  if (!canvasRef.value) return;

  const rect = canvasRef.value.getBoundingClientRect();
  dragStart.value = {
    x: event.clientX - rect.left,
    y: event.clientY - rect.top,
  };
  isDragging.value = true;
};

const handleMouseMove = (event: MouseEvent) => {
  if (!isDragging.value || !canvasRef.value) return;

  const rect = canvasRef.value.getBoundingClientRect();
  const x = event.clientX - rect.left;
  const y = event.clientY - rect.top;

  // Find node being dragged
  const draggedNode = props.nodes.find(node =>
    Math.abs(node.x - dragStart.value.x) < 50 &&
    Math.abs(node.y - dragStart.value.y) < 30
  );

  if (draggedNode) {
    move_node(draggedNode.id, x, y);
    emit('node-move', draggedNode.id, x, y);
  }
};

const handleMouseUp = () => {
  isDragging.value = false;
};
</script>

<style scoped>
.flow-container {
  position: relative;
  border: 1px solid #ccc;
  border-radius: 8px;
  overflow: hidden;
}

.loading {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  background: rgba(255, 255, 255, 0.9);
  padding: 20px;
  border-radius: 8px;
}
</style>
```

### Vue 3 with Pinia Store

```typescript
// stores/flowStore.ts
import { defineStore } from 'pinia';
import { ref, computed } from 'vue';

export interface FlowNode {
  id: string;
  x: number;
  y: number;
  label: string;
  color: string;
}

export interface FlowEdge {
  id: string;
  source: string;
  target: string;
}

export const useFlowStore = defineStore('flow', () => {
  const nodes = ref<FlowNode[]>([]);
  const edges = ref<FlowEdge[]>([]);
  const selectedNode = ref<string | null>(null);
  const isInitialized = ref(false);

  const nodeCount = computed(() => nodes.value.length);
  const edgeCount = computed(() => edges.value.length);

  const addNode = (node: FlowNode) => {
    nodes.value.push(node);
  };

  const removeNode = (nodeId: string) => {
    nodes.value = nodes.value.filter(node => node.id !== nodeId);
    edges.value = edges.value.filter(edge =>
      edge.source !== nodeId && edge.target !== nodeId
    );
    if (selectedNode.value === nodeId) {
      selectedNode.value = null;
    }
  };

  const moveNode = (nodeId: string, x: number, y: number) => {
    const node = nodes.value.find(n => n.id === nodeId);
    if (node) {
      node.x = x;
      node.y = y;
    }
  };

  const selectNode = (nodeId: string | null) => {
    selectedNode.value = nodeId;
  };

  const setInitialized = (value: boolean) => {
    isInitialized.value = value;
  };

  return {
    nodes,
    edges,
    selectedNode,
    isInitialized,
    nodeCount,
    edgeCount,
    addNode,
    removeNode,
    moveNode,
    selectNode,
    setInitialized,
  };
});
```

## Angular Integration

### Angular Component

```typescript
// flow-canvas.component.ts
import { Component, ElementRef, Input, Output, EventEmitter, OnInit, OnDestroy, ViewChild } from '@angular/core';
import init, {
  create_simple_flow,
  add_node_at_position,
  move_node
} from './pkg/simple_flow_example';

interface FlowNode {
  id: string;
  x: number;
  y: number;
  label: string;
  color: string;
}

@Component({
  selector: 'app-flow-canvas',
  template: `
    <div class="flow-container">
      <canvas
        #canvas
        [width]="width"
        [height]="height"
        (click)="onCanvasClick($event)"
        (mousedown)="onMouseDown($event)"
        (mousemove)="onMouseMove($event)"
        (mouseup)="onMouseUp($event)"
      ></canvas>
      <div *ngIf="!isInitialized" class="loading">
        Loading Flow Canvas...
      </div>
    </div>
  `,
  styleUrls: ['./flow-canvas.component.css']
})
export class FlowCanvasComponent implements OnInit, OnDestroy {
  @ViewChild('canvas', { static: true }) canvas!: ElementRef<HTMLCanvasElement>;

  @Input() width: number = 800;
  @Input() height: number = 600;
  @Input() nodes: FlowNode[] = [];

  @Output() nodeClick = new EventEmitter<string>();
  @Output() nodeMove = new EventEmitter<{nodeId: string, x: number, y: number}>();
  @Output() canvasClick = new EventEmitter<{x: number, y: number}>();

  isInitialized = false;
  private isDragging = false;
  private dragStart = { x: 0, y: 0 };

  async ngOnInit() {
    await init();
    this.isInitialized = true;

    if (this.canvas.nativeElement) {
      create_simple_flow();
    }
  }

  ngOnDestroy() {
    // Cleanup if needed
  }

  onCanvasClick(event: MouseEvent) {
    if (!this.canvas.nativeElement) return;

    const rect = this.canvas.nativeElement.getBoundingClientRect();
    const x = event.clientX - rect.left;
    const y = event.clientY - rect.top;

    // Check if clicking on a node
    const clickedNode = this.nodes.find(node =>
      Math.abs(node.x - x) < 50 && Math.abs(node.y - y) < 30
    );

    if (clickedNode) {
      this.nodeClick.emit(clickedNode.id);
    } else {
      this.canvasClick.emit({ x, y });
    }
  }

  onMouseDown(event: MouseEvent) {
    if (!this.canvas.nativeElement) return;

    const rect = this.canvas.nativeElement.getBoundingClientRect();
    this.dragStart = {
      x: event.clientX - rect.left,
      y: event.clientY - rect.top,
    };
    this.isDragging = true;
  }

  onMouseMove(event: MouseEvent) {
    if (!this.isDragging || !this.canvas.nativeElement) return;

    const rect = this.canvas.nativeElement.getBoundingClientRect();
    const x = event.clientX - rect.left;
    const y = event.clientY - rect.top;

    // Find node being dragged
    const draggedNode = this.nodes.find(node =>
      Math.abs(node.x - this.dragStart.x) < 50 &&
      Math.abs(node.y - this.dragStart.y) < 30
    );

    if (draggedNode) {
      move_node(draggedNode.id, x, y);
      this.nodeMove.emit({ nodeId: draggedNode.id, x, y });
    }
  }

  onMouseUp() {
    this.isDragging = false;
  }
}
```

### Angular Service

```typescript
// flow.service.ts
import { Injectable } from '@angular/core';
import { BehaviorSubject, Observable } from 'rxjs';
import init, {
  create_simple_flow,
  add_node_at_position,
  move_node,
  remove_node
} from './pkg/simple_flow_example';

export interface FlowNode {
  id: string;
  x: number;
  y: number;
  label: string;
  color: string;
}

export interface FlowEdge {
  id: string;
  source: string;
  target: string;
}

@Injectable({
  providedIn: 'root'
})
export class FlowService {
  private nodesSubject = new BehaviorSubject<FlowNode[]>([]);
  private edgesSubject = new BehaviorSubject<FlowEdge[]>([]);
  private selectedNodeSubject = new BehaviorSubject<string | null>(null);
  private isInitializedSubject = new BehaviorSubject<boolean>(false);

  public nodes$ = this.nodesSubject.asObservable();
  public edges$ = this.edgesSubject.asObservable();
  public selectedNode$ = this.selectedNodeSubject.asObservable();
  public isInitialized$ = this.isInitializedSubject.asObservable();

  constructor() {
    this.initializeWasm();
  }

  private async initializeWasm() {
    await init();
    this.isInitializedSubject.next(true);
  }

  addNode(node: FlowNode) {
    add_node_at_position(node.x, node.y);
    const currentNodes = this.nodesSubject.value;
    this.nodesSubject.next([...currentNodes, node]);
  }

  removeNode(nodeId: string) {
    remove_node(nodeId);
    const currentNodes = this.nodesSubject.value;
    const currentEdges = this.edgesSubject.value;

    this.nodesSubject.next(currentNodes.filter(node => node.id !== nodeId));
    this.edgesSubject.next(currentEdges.filter(edge =>
      edge.source !== nodeId && edge.target !== nodeId
    ));

    if (this.selectedNodeSubject.value === nodeId) {
      this.selectedNodeSubject.next(null);
    }
  }

  moveNode(nodeId: string, x: number, y: number) {
    move_node(nodeId, x, y);
    const currentNodes = this.nodesSubject.value;
    const updatedNodes = currentNodes.map(node =>
      node.id === nodeId ? { ...node, x, y } : node
    );
    this.nodesSubject.next(updatedNodes);
  }

  selectNode(nodeId: string | null) {
    this.selectedNodeSubject.next(nodeId);
  }

  getCurrentNodes(): FlowNode[] {
    return this.nodesSubject.value;
  }

  getCurrentEdges(): FlowEdge[] {
    return this.edgesSubject.value;
  }
}
```

## Vanilla JavaScript

### ES6 Module Integration

```javascript
// flow-manager.js
import init, {
  create_simple_flow,
  add_node_at_position,
  move_node,
  remove_node
} from './pkg/simple_flow_example.js';

class FlowManager {
  constructor(canvasId, options = {}) {
    this.canvasId = canvasId;
    this.canvas = document.getElementById(canvasId);
    this.options = {
      width: 800,
      height: 600,
      ...options
    };
    this.nodes = [];
    this.edges = [];
    this.selectedNode = null;
    this.isInitialized = false;
    this.isDragging = false;
    this.dragStart = { x: 0, y: 0 };

    this.init();
  }

  async init() {
    await init();
    this.isInitialized = true;

    if (this.canvas) {
      this.canvas.width = this.options.width;
      this.canvas.height = this.options.height;
      create_simple_flow();
      this.setupEventListeners();
    }
  }

  setupEventListeners() {
    this.canvas.addEventListener('click', this.handleCanvasClick.bind(this));
    this.canvas.addEventListener('mousedown', this.handleMouseDown.bind(this));
    this.canvas.addEventListener('mousemove', this.handleMouseMove.bind(this));
    this.canvas.addEventListener('mouseup', this.handleMouseUp.bind(this));
  }

  handleCanvasClick(event) {
    const rect = this.canvas.getBoundingClientRect();
    const x = event.clientX - rect.left;
    const y = event.clientY - rect.top;

    const clickedNode = this.nodes.find(node =>
      Math.abs(node.x - x) < 50 && Math.abs(node.y - y) < 30
    );

    if (clickedNode) {
      this.selectNode(clickedNode.id);
      this.onNodeClick?.(clickedNode);
    } else {
      this.selectNode(null);
      this.onCanvasClick?.(x, y);
    }
  }

  handleMouseDown(event) {
    const rect = this.canvas.getBoundingClientRect();
    this.dragStart = {
      x: event.clientX - rect.left,
      y: event.clientY - rect.top,
    };
    this.isDragging = true;
  }

  handleMouseMove(event) {
    if (!this.isDragging) return;

    const rect = this.canvas.getBoundingClientRect();
    const x = event.clientX - rect.left;
    const y = event.clientY - rect.top;

    const draggedNode = this.nodes.find(node =>
      Math.abs(node.x - this.dragStart.x) < 50 &&
      Math.abs(node.y - this.dragStart.y) < 30
    );

    if (draggedNode) {
      move_node(draggedNode.id, x, y);
      draggedNode.x = x;
      draggedNode.y = y;
      this.onNodeMove?.(draggedNode);
    }
  }

  handleMouseUp() {
    this.isDragging = false;
  }

  addNode(node) {
    add_node_at_position(node.x, node.y);
    this.nodes.push(node);
    this.onNodeAdded?.(node);
  }

  removeNode(nodeId) {
    remove_node(nodeId);
    this.nodes = this.nodes.filter(node => node.id !== nodeId);
    this.edges = this.edges.filter(edge =>
      edge.source !== nodeId && edge.target !== nodeId
    );

    if (this.selectedNode === nodeId) {
      this.selectedNode = null;
    }

    this.onNodeRemoved?.(nodeId);
  }

  selectNode(nodeId) {
    this.selectedNode = nodeId;
    this.onNodeSelected?.(nodeId);
  }

  // Event callbacks
  onNodeClick = null;
  onNodeMove = null;
  onCanvasClick = null;
  onNodeAdded = null;
  onNodeRemoved = null;
  onNodeSelected = null;
}

// Usage
const flowManager = new FlowManager('flow-canvas', {
  width: 1000,
  height: 600
});

flowManager.onNodeClick = (node) => {
  console.log('Node clicked:', node);
};

flowManager.onCanvasClick = (x, y) => {
  const newNode = {
    id: `node_${Date.now()}`,
    x,
    y,
    label: `Node ${flowManager.nodes.length + 1}`,
    color: '#3b82f6'
  };
  flowManager.addNode(newNode);
};

export default FlowManager;
```

## WebSocket Real-time Updates

### WebSocket Integration

```javascript
// websocket-flow.js
import FlowManager from './flow-manager.js';

class WebSocketFlowManager extends FlowManager {
  constructor(canvasId, wsUrl, options = {}) {
    super(canvasId, options);
    this.wsUrl = wsUrl;
    this.ws = null;
    this.reconnectAttempts = 0;
    this.maxReconnectAttempts = 5;
    this.reconnectDelay = 1000;

    this.connect();
  }

  connect() {
    try {
      this.ws = new WebSocket(this.wsUrl);

      this.ws.onopen = () => {
        console.log('WebSocket connected');
        this.reconnectAttempts = 0;
        this.onConnected?.();
      };

      this.ws.onmessage = (event) => {
        const data = JSON.parse(event.data);
        this.handleWebSocketMessage(data);
      };

      this.ws.onclose = () => {
        console.log('WebSocket disconnected');
        this.onDisconnected?.();
        this.attemptReconnect();
      };

      this.ws.onerror = (error) => {
        console.error('WebSocket error:', error);
        this.onError?.(error);
      };
    } catch (error) {
      console.error('Failed to connect to WebSocket:', error);
      this.attemptReconnect();
    }
  }

  handleWebSocketMessage(data) {
    switch (data.type) {
      case 'node_added':
        this.addNodeFromRemote(data.node);
        break;
      case 'node_moved':
        this.moveNodeFromRemote(data.nodeId, data.x, data.y);
        break;
      case 'node_removed':
        this.removeNodeFromRemote(data.nodeId);
        break;
      case 'nodes_sync':
        this.syncNodes(data.nodes);
        break;
    }
  }

  addNodeFromRemote(node) {
    // Add node without triggering WebSocket message
    add_node_at_position(node.x, node.y);
    this.nodes.push(node);
    this.onNodeAdded?.(node);
  }

  moveNodeFromRemote(nodeId, x, y) {
    // Move node without triggering WebSocket message
    move_node(nodeId, x, y);
    const node = this.nodes.find(n => n.id === nodeId);
    if (node) {
      node.x = x;
      node.y = y;
      this.onNodeMove?.(node);
    }
  }

  removeNodeFromRemote(nodeId) {
    // Remove node without triggering WebSocket message
    remove_node(nodeId);
    this.nodes = this.nodes.filter(node => node.id !== nodeId);
    this.edges = this.edges.filter(edge =>
      edge.source !== nodeId && edge.target !== nodeId
    );
    this.onNodeRemoved?.(nodeId);
  }

  syncNodes(remoteNodes) {
    // Clear current nodes
    this.nodes.forEach(node => remove_node(node.id));
    this.nodes = [];

    // Add remote nodes
    remoteNodes.forEach(node => {
      add_node_at_position(node.x, node.y);
      this.nodes.push(node);
    });

    this.onNodesSynced?.(remoteNodes);
  }

  sendMessage(type, data) {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify({ type, ...data }));
    }
  }

  addNode(node) {
    super.addNode(node);
    this.sendMessage('node_added', { node });
  }

  removeNode(nodeId) {
    super.removeNode(nodeId);
    this.sendMessage('node_removed', { nodeId });
  }

  handleMouseMove(event) {
    if (!this.isDragging) return;

    const rect = this.canvas.getBoundingClientRect();
    const x = event.clientX - rect.left;
    const y = event.clientY - rect.top;

    const draggedNode = this.nodes.find(node =>
      Math.abs(node.x - this.dragStart.x) < 50 &&
      Math.abs(node.y - this.dragStart.y) < 30
    );

    if (draggedNode) {
      move_node(draggedNode.id, x, y);
      draggedNode.x = x;
      draggedNode.y = y;
      this.sendMessage('node_moved', {
        nodeId: draggedNode.id,
        x,
        y
      });
      this.onNodeMove?.(draggedNode);
    }
  }

  attemptReconnect() {
    if (this.reconnectAttempts < this.maxReconnectAttempts) {
      this.reconnectAttempts++;
      setTimeout(() => {
        console.log(`Attempting to reconnect (${this.reconnectAttempts}/${this.maxReconnectAttempts})`);
        this.connect();
      }, this.reconnectDelay * this.reconnectAttempts);
    }
  }

  disconnect() {
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
  }

  // Event callbacks
  onConnected = null;
  onDisconnected = null;
  onError = null;
  onNodesSynced = null;
}

// Usage
const wsFlowManager = new WebSocketFlowManager(
  'flow-canvas',
  'ws://localhost:8080/ws'
);

wsFlowManager.onConnected = () => {
  console.log('Connected to collaborative flow');
};

wsFlowManager.onNodeAdded = (node) => {
  console.log('Node added:', node);
};

wsFlowManager.onNodesSynced = (nodes) => {
  console.log('Nodes synced:', nodes.length);
};

export default WebSocketFlowManager;
```

## REST API Integration

### REST API Client

```javascript
// api-flow.js
import FlowManager from './flow-manager.js';

class APIFlowManager extends FlowManager {
  constructor(canvasId, apiBaseUrl, options = {}) {
    super(canvasId, options);
    this.apiBaseUrl = apiBaseUrl;
    this.isLoading = false;
  }

  async loadFlow(flowId) {
    try {
      this.isLoading = true;
      const response = await fetch(`${this.apiBaseUrl}/flows/${flowId}`);
      const data = await response.json();

      // Clear current flow
      this.nodes.forEach(node => remove_node(node.id));
      this.nodes = [];
      this.edges = [];

      // Load nodes
      data.nodes.forEach(node => {
        add_node_at_position(node.x, node.y);
        this.nodes.push(node);
      });

      // Load edges
      this.edges = data.edges;

      this.onFlowLoaded?.(data);
    } catch (error) {
      console.error('Failed to load flow:', error);
      this.onError?.(error);
    } finally {
      this.isLoading = false;
    }
  }

  async saveFlow(flowId, metadata = {}) {
    try {
      this.isLoading = true;
      const flowData = {
        id: flowId,
        nodes: this.nodes,
        edges: this.edges,
        metadata,
        updatedAt: new Date().toISOString()
      };

      const response = await fetch(`${this.apiBaseUrl}/flows/${flowId}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(flowData)
      });

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      const result = await response.json();
      this.onFlowSaved?.(result);
    } catch (error) {
      console.error('Failed to save flow:', error);
      this.onError?.(error);
    } finally {
      this.isLoading = false;
    }
  }

  async createFlow(name, description = '') {
    try {
      this.isLoading = true;
      const flowData = {
        name,
        description,
        nodes: this.nodes,
        edges: this.edges,
        createdAt: new Date().toISOString()
      };

      const response = await fetch(`${this.apiBaseUrl}/flows`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(flowData)
      });

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      const result = await response.json();
      this.onFlowCreated?.(result);
      return result;
    } catch (error) {
      console.error('Failed to create flow:', error);
      this.onError?.(error);
      throw error;
    } finally {
      this.isLoading = false;
    }
  }

  async listFlows() {
    try {
      const response = await fetch(`${this.apiBaseUrl}/flows`);
      const flows = await response.json();
      this.onFlowsListed?.(flows);
      return flows;
    } catch (error) {
      console.error('Failed to list flows:', error);
      this.onError?.(error);
      throw error;
    }
  }

  async deleteFlow(flowId) {
    try {
      const response = await fetch(`${this.apiBaseUrl}/flows/${flowId}`, {
        method: 'DELETE'
      });

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      this.onFlowDeleted?.(flowId);
    } catch (error) {
      console.error('Failed to delete flow:', error);
      this.onError?.(error);
      throw error;
    }
  }

  // Event callbacks
  onFlowLoaded = null;
  onFlowSaved = null;
  onFlowCreated = null;
  onFlowsListed = null;
  onFlowDeleted = null;
  onError = null;
}

// Usage
const apiFlowManager = new APIFlowManager(
  'flow-canvas',
  'http://localhost:3000/api'
);

apiFlowManager.onFlowLoaded = (flow) => {
  console.log('Flow loaded:', flow.name);
};

apiFlowManager.onFlowSaved = (result) => {
  console.log('Flow saved:', result.id);
};

// Load a flow
apiFlowManager.loadFlow('flow-123');

// Save current flow
apiFlowManager.saveFlow('flow-123', {
  name: 'My Flow',
  description: 'A sample flow diagram'
});

export default APIFlowManager;
```

These integration examples demonstrate how to use the Leptos Flow Simple Example with various frameworks and technologies, providing a solid foundation for building flow-based applications in different environments.
