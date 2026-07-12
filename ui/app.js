const state = {
    project: createDefaultProject(),
    activeTab: 'project',
    activeNarrativeId: null,
    activeCharacterId: null,
    activePlotId: null,
    storyFilterType: "All",
    activity: [],
    agents: [],
    selectedAgentId: null,
    selectedAgentName: null,
    catalogs: {
        generic: null,
        generated: []
    },
    selectedCatalog: 'generic',
    selectedStoryElementType: 'All',
    highlightedStoryElements: []
};

function createDefaultProject() {
  return {
    projectMeta: {
      id: '',
      title: 'Nuevo Proyecto Cadiz12',
      summary: '',
      format: 'theatre_play',
      worldContext: '',
      allowedGenres: [],
      toneProfile: { seriousnessMin: 1, seriousnessMax: 5, darknessMin: 1, darknessMax: 5 },
      contentLimits: { maxRating: 'PG-13', blockedSensitivityTags: [] },
      productionConstraints: { maxCastSize: null, maxLocations: null, budgetBand: 'medium' },
      notes: ''
    },
    projectCharacters: [],
    narratives: [{
      id: 'nar_001',
      title: 'Primera narrativa',
      summary: '',
      description: '',
      act: 'Act1',
      tone: 'Ambiguous',
      historicalscope: 'PlausibleInferred',
      spaces: [],
      factions: [],
      stakes: [],
      tags: [],
      extendedNotes: '',
      futureCompat: {},
      cast: {
        rules: { protagonistMode: 'single', maxAntagonists: 2, maxSupporting: 4, allowExternalCharacters: true, allowRoleOverride: true },
        entries: []
      }
    }],
    plots: [],
    storyelements: [],
    events: [],
    review: { summary: '', issues: [] },
    aihistory: [],
    settings: {
      agents: {
        project: 'CoordinadorNarrativo',
        narrative: 'CoordinadorNarrativo',
        characters: 'CoordinadorNarrativo',
        plots: 'DiseñadorDeStoryElements',
        storyelements: 'DiseñadorDeStoryElements',
        events: 'DiseñadorDeEventos',
        validation: 'RevisorNarrativo',
        export: 'EditorDeExportacion'
      }
    }
  };
}

function $(selector) {
    return document.querySelector(selector);
}

function $all(selector) {
    return Array.from(document.querySelectorAll(selector));
}

function escapeHtml(text) {
    return String(text ?? "")
        .replace(/&/g, "&amp;")
        .replace(/</g, "&lt;")
        .replace(/>/g, "&gt;")
        .replace(/"/g, "&quot;")
        .replace(/'/g, "&#039;");
}

function safeJsonParse(text, fallback) {
    try {
        return JSON.parse(text);
    } catch (_) {
        return fallback;
    }
}

function clone(value) {
    return JSON.parse(JSON.stringify(value));
}

function textToLines(text) {
    return String(text || "")
        .split("\n")
        .map((line) => line.trim())
        .filter(Boolean);
}

function linesToText(values) {
    return Array.isArray(values) ? values.join("\n") : "";
}

function generateId(prefix) {
    return `${prefix}_${Date.now()}_${Math.floor(Math.random() * 100000)}`;
}

function ensureProjectShape(raw) {
  const p = raw && typeof raw === 'object' ? clone(raw) : createDefaultProject();

  if (p.project && !p.projectMeta) {
    p.projectMeta = {
      id: '',
      title: p.project.title || '',
      summary: p.project.summary || '',
      format: 'theatre_play',
      worldContext: '',
      allowedGenres: [],
      toneProfile: { seriousnessMin: 1, seriousnessMax: 5, darknessMin: 1, darknessMax: 5 },
      contentLimits: { maxRating: 'PG-13', blockedSensitivityTags: [] },
      productionConstraints: { maxCastSize: null, maxLocations: null, budgetBand: 'medium' },
      notes: ''
    };
  }
  if (p.project && (!Array.isArray(p.narratives) || !p.narratives.length)) {
    p.narratives = [{
      id: 'nar_001',
      title: p.project.title || 'Primera narrativa',
      summary: p.project.summary || '',
      description: '',
      act: p.project.act || 'Act1',
      tone: p.project.tone || 'Ambiguous',
      historicalscope: p.project.historicalscope || 'PlausibleInferred',
      spaces: p.project.spaces || [],
      factions: p.project.factions || [],
      stakes: p.project.stakes || [],
      tags: p.project.tags || [],
      extendedNotes: '',
      futureCompat: {},
      cast: { rules: { protagonistMode: 'single', maxAntagonists: 2, maxSupporting: 4 }, entries: [] }
    }];
  }
  if (p.storyelements && !p.plots) {
    p.legacyStoryElements = p.storyelements;
  }

  const base = createDefaultProject();

  p.projectMeta = { ...base.projectMeta, ...(p.projectMeta || {}) };
  p.projectCharacters = Array.isArray(p.projectCharacters) ? p.projectCharacters : [];
  p.narratives = Array.isArray(p.narratives) ? p.narratives : [];
  p.plots = Array.isArray(p.plots) ? p.plots : [];
  p.storyelements = Array.isArray(p.storyelements) ? p.storyelements : [];
  p.events = Array.isArray(p.events) ? p.events : [];
  p.review = { ...base.review, ...(p.review || {}) };
  p.aihistory = Array.isArray(p.aihistory) ? p.aihistory : (Array.isArray(p.aiHistory) ? p.aiHistory : []);
  p.settings = p.settings || {};
  p.settings.agents = { ...base.settings.agents, ...((p.settings && p.settings.agents) || {}) };

  if (!p.narratives.length) {
    p.narratives.push(clone(base.narratives[0]));
  }

  p.narratives = p.narratives.map((n, idx) => ({
    id: n.id || `nar_${idx + 1}`,
    title: n.title || 'Sin título',
    summary: n.summary || '',
    description: n.description || '',
    act: n.act || 'Act1',
    tone: n.tone || 'Ambiguous',
    historicalscope: n.historicalscope || 'PlausibleInferred',
    spaces: Array.isArray(n.spaces) ? n.spaces : [],
    factions: Array.isArray(n.factions) ? n.factions : [],
    stakes: Array.isArray(n.stakes) ? n.stakes : [],
    tags: Array.isArray(n.tags) ? n.tags : [],
    extendedNotes: n.extendedNotes || '',
    futureCompat: n.futureCompat || {},
    cast: n.cast || { rules: { protagonistMode: 'single', maxAntagonists: 2, maxSupporting: 4 }, entries: [] }
  }));

  p.projectCharacters = p.projectCharacters.map((c, idx) => ({
    id: c.id || `char_${idx + 1}`,
    name: c.name || 'Sin nombre',
    projectRoleType: c.projectRoleType || 'flex',
    summary: c.summary || '',
    description: c.description || '',
    extendedNotes: c.extendedNotes || '',
    futureCompat: c.futureCompat || {},
    narrativeProfile: {
      dramaticFunctions: Array.isArray(c.narrativeProfile?.dramaticFunctions) ? c.narrativeProfile.dramaticFunctions : [],
      coreDrives: Array.isArray(c.narrativeProfile?.coreDrives) ? c.narrativeProfile.coreDrives : [],
      traits: Array.isArray(c.narrativeProfile?.traits) ? c.narrativeProfile.traits : [],
      toneFit: Array.isArray(c.narrativeProfile?.toneFit) ? c.narrativeProfile.toneFit : []
    },
    worldFit: {
      allowedPeriods: Array.isArray(c.worldFit?.allowedPeriods) ? c.worldFit.allowedPeriods : [],
      preferredSettings: Array.isArray(c.worldFit?.preferredSettings) ? c.worldFit.preferredSettings : [],
      genreAffinity: Array.isArray(c.worldFit?.genreAffinity) ? c.worldFit.genreAffinity : []
    },
    relationships: Array.isArray(c.relationships) ? c.relationships : []
  }));

  p.plots = p.plots.map((pl, idx) => ({
    id: pl.id || `plot_${idx + 1}`,
    title: pl.title || 'Sin título',
    narrativeId: pl.narrativeId || (p.narratives[0]?.id || ''),
    summary: pl.summary || '',
    status: pl.status || 'draft',
    extendedNotes: pl.extendedNotes || '',
    futureCompat: pl.futureCompat || {},
    characterSelection: {
      protagonistCharacterId: pl.characterSelection?.protagonistCharacterId || null,
      antagonistCharacterIds: Array.isArray(pl.characterSelection?.antagonistCharacterIds) ? pl.characterSelection.antagonistCharacterIds : [],
      supportingCharacterIds: Array.isArray(pl.characterSelection?.supportingCharacterIds) ? pl.characterSelection.supportingCharacterIds : [],
      entries: Array.isArray(pl.characterSelection?.entries) ? pl.characterSelection.entries : []
    },
    selection: {
      themeIds: Array.isArray(pl.selection?.themeIds) ? pl.selection.themeIds : [],
      eventIds: Array.isArray(pl.selection?.eventIds) ? pl.selection.eventIds : [],
      settingIds: Array.isArray(pl.selection?.settingIds) ? pl.selection.settingIds : [],
      finaleId: pl.selection?.finaleId || null,
      genreIds: Array.isArray(pl.selection?.genreIds) ? pl.selection.genreIds : []
    }
  }));

  return p;
}

function setStatus(message, type = "info") {
    const banner = $("#statusBanner");
    banner.textContent = message;
    banner.className = `status-banner ${type}`;
    banner.classList.remove("hidden");
}

function clearStatus() {
    const banner = $("#statusBanner");
    banner.textContent = "";
    banner.className = "status-banner hidden";
}

function addActivity(entry) {
    state.activity.unshift({
        timestamp: new Date().toLocaleString("es-ES"),
        ...entry
    });
    renderActivity();
}

function renderActivity() {
    const container = $("#activityLog");
    if (!state.activity.length) {
        container.className = "activity-log empty-state";
        container.innerHTML = "Aún no hay actividad.";
        return;
    }

    container.className = "activity-log";
    container.innerHTML = state.activity.slice(0, 20).map((item) => `
        <article class="activity-item">
            <div class="activity-topline">
                <strong>${escapeHtml(item.label || "Actividad")}</strong>
                <span>${escapeHtml(item.timestamp || "")}</span>
            </div>
            <div class="activity-meta">
                ${escapeHtml(item.detail || "")}
            </div>
        </article>
    `).join("");
}

async function apiGet(url) {
    const response = await fetch(url, { cache: "no-store" });
    const text = await response.text();
    const json = safeJsonParse(text, null);

    if (!response.ok) {
        throw new Error((json && json.message) || text || `HTTP ${response.status}`);
    }

    return json;
}

async function apiPost(url, payload) {
    const response = await fetch(url, {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify(payload)
    });

    const text = await response.text();
    const json = safeJsonParse(text, null);

    if (!response.ok) {
        throw new Error((json && json.message) || text || `HTTP ${response.status}`);
    }

    return json;
}

async function loadAgents() {
    try {
        const payload = await apiGet("/api/agents");
        state.agents = Array.isArray(payload?.data) ? payload.data : [];
        renderAgents();
    } catch (error) {
        state.agents = [];
        renderAgents();
        addActivity({
            label: "Agentes",
            detail: `Error cargando agentes: ${error.message}`
        });
    }
}

function renderAgents() {
    // Llenar el selector global de agentes
    renderAgentSelectors();
}

function renderAgentSelectors() {
    const globalSelect = $("#selectedAgent");
    if (!globalSelect) return;
    
    const options = ['<option value="">-- Selecciona un agente --</option>'];
    state.agents.forEach(agent => {
        const selected = agent.agent_id === state.selectedAgentId ? 'selected' : '';
        options.push(`<option value="${escapeHtml(agent.agent_id)}" data-name="${escapeHtml(agent.nombre)}" ${selected}>${escapeHtml(agent.nombre)}</option>`);
    });
    globalSelect.innerHTML = options.join("");
    
    // Restaurar selección si existe
    if (state.selectedAgentId) {
        globalSelect.value = state.selectedAgentId;
    }
}

async function saveAgent() {
    const name = $("#agentName")?.value?.trim() || "";
    const agentId = $("#agentId")?.value?.trim() || "";
    const conversationId = $("#agentConversationId")?.value?.trim() || "";
    
    if (!name || !agentId) {
        setStatus("Nombre e ID de agente son obligatorios.", "error");
        return;
    }
    
    setStatus("Guardando agente...", "info");
    
    try {
        const response = await fetch("http://127.0.0.1:8000/agentes", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ nombre: name, agent_id: agentId, conversation_id: conversationId })
        });
        
        if (!response.ok) {
            const error = await response.text();
            throw new Error(error || "Error al guardar agente");
        }
        
        await loadAgents();
        setStatus("Agente guardado correctamente.", "success");
        
        // Limpiar formulario
        $("#agentName").value = "";
        $("#agentId").value = "";
        $("#agentConversationId").value = "";
        
    } catch (error) {
        setStatus(`Error guardando agente: ${error.message}`, "error");
    }
}

async function deleteAgent() {
    const agentId = $("#agentId")?.value?.trim() || state.selectedAgentId;
    
    if (!agentId) {
        setStatus("No hay agente seleccionado para eliminar.", "error");
        return;
    }
    
    if (!confirm(`¿Estás seguro de que quieres eliminar el agente con ID: ${agentId}?`)) {
        return;
    }
    
    setStatus("Eliminando agente...", "info");
    
    try {
        const response = await fetch(`http://127.0.0.1:8000/agentes/${encodeURIComponent(agentId)}`, {
            method: "DELETE"
        });
        
        if (!response.ok) {
            const error = await response.text();
            throw new Error(error || "Error al eliminar agente");
        }
        
        await loadAgents();
        setStatus("Agente eliminado correctamente.", "success");
        
        // Limpiar selección
        state.selectedAgentId = null;
        state.selectedAgentName = null;
        $("#agentName").value = "";
        $("#agentId").value = "";
        $("#agentConversationId").value = "";
        
    } catch (error) {
        setStatus(`Error eliminando agente: ${error.message}`, "error");
    }
}

async function resetAgentConversation() {
    const agentId = $("#agentId")?.value?.trim() || state.selectedAgentId;
    
    if (!agentId) {
        setStatus("No hay agente seleccionado.", "error");
        return;
    }
    
    setStatus("Reseteando conversación...", "info");
    
    try {
        const response = await fetch("http://127.0.0.1:8000/agentes/reset_conversation", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ agent_id: agentId })
        });
        
        if (!response.ok) {
            const error = await response.text();
            throw new Error(error || "Error al resetear conversación");
        }
        
        await loadAgents();
        setStatus("Conversación reseteada. El siguiente mensaje creará una nueva.", "success");
        
    } catch (error) {
        setStatus(`Error reseteando conversación: ${error.message}`, "error");
    }
}

function selectAgentFromTab(agentId, name) {
    state.selectedAgentId = agentId;
    state.selectedAgentName = name;
    renderAgentSelectors();
    setStatus(`Agente seleccionado: ${name}`, "success");
}

function renderAgentsTab() {
    const container = $("#agentsList");
    if (!container) return;
    
    if (!state.agents.length) {
        container.className = "card-list empty-state";
        container.innerHTML = "No hay agentes configurados. Añade uno usando el botón + Añadir Agente.";
        return;
    }
    
    container.className = "card-list";
    container.innerHTML = state.agents.map(agent => {
        const isSelected = agent.agent_id === state.selectedAgentId;
        return `
            <article class="entity-card ${isSelected ? 'selected' : ''}" style="cursor: pointer;" 
                     onclick="selectAgentFromTab('${escapeHtml(agent.agent_id)}', '${escapeHtml(agent.nombre)}')">
                <div class="entity-card-header">
                    <div class="entity-card-title-group">
                        <h4 style="margin: 0;">${escapeHtml(agent.nombre)}</h4>
                        <span class="badge" style="font-size: 0.75em;">${escapeHtml(agent.agent_id)}</span>
                    </div>
                </div>
                <div class="form-grid two-cols compact-grid" style="margin-top: 0.5em;">
                    <div class="field">
                        <label>Conversation ID</label>
                        <div class="mono" style="font-size: 0.85em;">${escapeHtml(agent.conversation_id || 'Ninguna')}</div>
                    </div>
                    <div class="field">
                        <label>Último uso</label>
                        <div style="font-size: 0.85em;">${escapeHtml(agent.last_used_at || 'Nunca')}</div>
                    </div>
                </div>
            </article>
        `;
    }).join("");
}

// ============================================
// FUNCIONES PARA CATÁLOGOS DE STORY ELEMENTS
// ============================================

async function loadGenericCatalog() {
    try {
        // Cargar todos los CSV del catálogo genérico
        const catalogFiles = [
            'protagonist', 'antagonist', 'theme', 'secondary',
            'scenario', 'procedure', 'dramaticresource', 'genre',
            'settings', 'finale', 'events'
        ];
        
        const allElements = [];
        
        for (const file of catalogFiles) {
            try {
                const response = await fetch(`/catalogs/${file}.csv`);
                if (response.ok) {
                    const csv = await response.text();
                    const elements = parseCSV(csv, file);
                    allElements.push(...elements);
                }
            } catch (error) {
                console.warn(`No se pudo cargar ${file}.csv:`, error);
            }
        }
        
        state.catalogs.generic = allElements;
        console.log('[DEBUG] Catálogo genérico cargado:', allElements.length, 'elementos');
        if (allElements.length > 0) {
            console.log('[DEBUG] Primer elemento:', allElements[0]);
        }
        return allElements;
    } catch (error) {
        console.error('[ERROR] Error cargando catálogo genérico:', error);
        return [];
    }
}

function parseCSV(csv, type) {
    const lines = csv.split('\n');
    if (lines.length < 2) return [];
    
    const headers = lines[0].split(',').map(h => h.trim());
    const elements = [];
    
    for (let i = 1; i < lines.length; i++) {
        const line = lines[i].trim();
        if (!line) continue;
        
        // Usar parser CSV robusto que maneje comillas
        const values = parseCSVLine(line);
        const element = { type: type };
        
        for (let j = 0; j < Math.min(headers.length, values.length); j++) {
            element[headers[j]] = values[j];
        }
        
        elements.push(element);
    }
    
    return elements;
}

function parseCSVLine(line) {
    const values = [];
    let current = '';
    let inQuotes = false;
    
    for (let i = 0; i < line.length; i++) {
        const char = line[i];
        
        if (char === '"') {
            inQuotes = !inQuotes;
        } else if (char === ',' && !inQuotes) {
            values.push(current.trim());
            current = '';
        } else {
            current += char;
        }
    }
    
    values.push(current.trim());
    return values;
}

function renderCatalogSelector() {
    const select = document.getElementById('catalogSelect');
    if (!select) return;
    
    const options = [
        '<option value="generic">Catálogo Genérico (CSV)</option>',
        '<option value="">-- Catálogos Generados por IA --</option>'
    ];
    
    state.catalogs.generated.forEach((catalog, index) => {
        options.push(`<option value="generated_${index}">Catálogo ${index + 1} (${catalog.name || 'sin nombre'})</option>`);
    });
    
    select.innerHTML = options.join('');
    select.value = state.selectedCatalog;
}

function renderCatalogStoryElements() {
    const container = document.getElementById('catalogStoryElementsList');
    if (!container) return;
    
    let elements = [];
    
    if (state.selectedCatalog === 'generic') {
        elements = state.catalogs.generic || [];
    } else if (state.selectedCatalog.startsWith('generated_')) {
        const index = parseInt(state.selectedCatalog.replace('generated_', ''));
        elements = state.catalogs.generated[index]?.elements || [];
    }
    
    // Filtrar por tipología
    const typeFilter = state.selectedStoryElementType;
    if (typeFilter !== 'All') {
        elements = elements.filter(el => el.type === typeFilter.toLowerCase() || el.category === typeFilter);
    }
    
    if (!elements.length) {
        container.className = 'card-list empty-state';
        container.innerHTML = 'No hay story elements para este catálogo y filtro.';
        return;
    }
    
    container.className = 'card-list';
    container.innerHTML = elements.map(element => {
        const isHighlighted = state.highlightedStoryElements.includes(element.id);
        const highlightClass = isHighlighted ? 'highlighted' : '';
        
        return `
            <article class="entity-card story-card ${highlightClass}" data-element-id="${element.id || ''}">
                <div class="entity-card-header">
                    <div class="entity-card-title-group">
                        <h4 style="margin: 0;">${escapeHtml(element.spanish_name || element.english_name || element.id || 'Sin nombre')}</h4>
                        <span class="badge" style="font-size: 0.75em;">${escapeHtml(element.type || element.category || 'Unknown')}</span>
                    </div>
                </div>
                <div class="form-grid two-cols compact-grid" style="margin-top: 0.5em;">
                    <div class="field">
                        <label>ID</label>
                        <div class="mono" style="font-size: 0.85em;">${escapeHtml(element.id || '')}</div>
                    </div>
                    <div class="field">
                        <label>Tipo</label>
                        <div style="font-size: 0.85em;">${escapeHtml(element.subtype || element.role_in_story || '')}</div>
                    </div>
                    <div class="field field-full">
                        <label>Descripción</label>
                        <div style="font-size: 0.85em; max-height: 100px; overflow: hidden;">${escapeHtml((element.logline_usage || element.description || '').substring(0, 200))}</div>
                    </div>
                </div>
            </article>
        `;
    }).join('');
}

async function generateAdaptedCatalog() {
    setStatus('Generando catálogo adaptado...', 'info');
    
    try {
        // Preparar el payload con el proyecto actual
        const payload = {
            action: 'generate_catalog',
            project: state.project,
            generic_catalog: state.catalogs.generic
        };
        
        // Llamar a la IA para generar un catálogo adaptado
        const response = await fetch('http://127.0.0.1:8000/enviar_mensaje', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                agente: state.selectedAgentName || 'CoordinadorNarrativo',
                mensaje: `Generar un catálogo de story elements adaptado al siguiente proyecto: ${JSON.stringify(state.project.projectMeta, null, 2)}. 

Basado en el catálogo genérico: ${JSON.stringify(state.catalogs.generic.slice(0, 5), null, 2)}...

Devuelve SOLO JSON con el formato: {"name": "nombre del catálogo", "elements": [{"id": "...", "name": "...", "type": "...", "description": "...", "recommended": true/false}]}`
            })
        });
        
        if (!response.ok) {
            throw new Error('Error al generar catálogo');
        }
        
        const result = await response.json();
        
        if (result.status === 'success') {
            // Parsear la respuesta (asumiendo que es JSON válido)
            let catalogData;
            try {
                catalogData = typeof result.text === 'string' ? JSON.parse(result.text) : result.text;
            } catch (e) {
                catalogData = { name: 'Catálogo Generado', elements: [] };
            }
            
            // Añadir al estado
            state.catalogs.generated.push(catalogData);
            state.selectedCatalog = `generated_${state.catalogs.generated.length - 1}`;
            
            // Renderizar
            renderCatalogSelector();
            renderCatalogStoryElements();
            
            setStatus(`Catálogo "${catalogData.name}" generado correctamente.`, 'success');
        } else {
            setStatus(`Error: ${result.message || 'Respuesta inválida'}`, 'error');
        }
    } catch (error) {
        setStatus(`Error generando catálogo: ${error.message}`, 'error');
    }
}

async function highlightRecommendedElements() {
    setStatus('Pidiendo recomendaciones a la IA...', 'info');
    
    try {
        let elements = [];
        if (state.selectedCatalog === 'generic') {
            elements = state.catalogs.generic || [];
        } else if (state.selectedCatalog.startsWith('generated_')) {
            const index = parseInt(state.selectedCatalog.replace('generated_', ''));
            elements = state.catalogs.generated[index]?.elements || [];
        }
        
        // Preparar el payload
        const payload = {
            action: 'recommend_elements',
            project: state.project,
            elements: elements
        };
        
        // Llamar a la IA para resaltar elementos recomendados
        const response = await fetch('http://127.0.0.1:8000/enviar_mensaje', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                agente: state.selectedAgentName || 'ContextoHistorico',
                mensaje: `Analiza el siguiente proyecto: ${JSON.stringify(state.project.projectMeta, null, 2)}. 

De los siguientes story elements, selecciona los más adecuados: ${JSON.stringify(elements.slice(0, 10), null, 2)}...

Devuelve SOLO JSON con un array de IDs recomendados: ["id1", "id2", ...]`
            })
        });
        
        if (!response.ok) {
            throw new Error('Error al obtener recomendaciones');
        }
        
        const result = await response.json();
        
        if (result.status === 'success') {
            // Parsear la respuesta
            let recommendedIds;
            try {
                recommendedIds = typeof result.text === 'string' ? JSON.parse(result.text) : result.text;
            } catch (e) {
                recommendedIds = [];
            }
            
            // Guardar los IDs recomendados
            state.highlightedStoryElements = Array.isArray(recommendedIds) ? recommendedIds : [];
            
            // Renderizar para mostrar los resaltados
            renderCatalogStoryElements();
            
            setStatus(`Elementos recomendados resaltados (${state.highlightedStoryElements.length}).`, 'success');
        } else {
            setStatus(`Error: ${result.message || 'Respuesta inválida'}`, 'error');
        }
    } catch (error) {
        setStatus(`Error obteniendo recomendaciones: ${error.message}`, 'error');
    }
}

// ============================================
// FUNCIONES ESPECÍFICAS PARA TRAMAS CON CATÁLOGO
// ============================================

async function generatePlotFromCatalog() {
    if (!state.selectedAgentId) {
        setStatus('Selecciona un agente primero.', 'error');
        return;
    }
    
    if (!state.selectedCatalog) {
        setStatus('Selecciona un catálogo primero.', 'error');
        return;
    }
    
    setStatus('Generando trama desde catálogo...', 'info');
    
    try {
        // Obtener los elementos del catálogo seleccionado
        let catalogElements = [];
        if (state.selectedCatalog === 'generic') {
            catalogElements = state.catalogs.generic || [];
        } else if (state.selectedCatalog.startsWith('generated_')) {
            const index = parseInt(state.selectedCatalog.replace('generated_', ''));
            catalogElements = state.catalogs.generated[index]?.elements || [];
        }
        
        // Filtrar por elementos recomendados si los hay
        const filteredElements = state.highlightedStoryElements.length > 0
            ? catalogElements.filter(el => state.highlightedStoryElements.includes(el.id))
            : catalogElements;
        
        // Llamar a la IA para generar una trama
        const response = await fetch('http://127.0.0.1:8000/enviar_mensaje', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                agente: state.selectedAgentName || 'CoordinadorNarrativo',
                mensaje: `Generar una trama completa para el proyecto: ${JSON.stringify(state.project.projectMeta, null, 2)}. 

Usa los siguientes story elements del catálogo: ${JSON.stringify(filteredElements.slice(0, 20), null, 2)}. 

Devuelve SOLO JSON con el formato de trama: {"id": "...", "title": "...", "summary": "...", "status": "draft", "protagonist_id": "...", "antagonist_ids": [...], "theme_ids": [...], "event_ids": [...], "setting_ids": [...], "finale_id": "..."}`
            })
        });
        
        if (!response.ok) {
            throw new Error('Error al generar trama');
        }
        
        const result = await response.json();
        
        if (result.status === 'success') {
            // Parsear la respuesta
            let plotData;
            try {
                plotData = typeof result.text === 'string' ? JSON.parse(result.text) : result.text;
            } catch (e) {
                setStatus(`Error parseando respuesta: ${e.message}`, 'error');
                return;
            }
            
            // Añadir la trama al proyecto
            state.project.plots.push(plotData);
            
            // Renderizar
            renderPlotList();
            renderProjectMeta();
            renderRawJson();
            
            setStatus(`Trama "${plotData.title}" generada desde catálogo.`, 'success');
        } else {
            setStatus(`Error: ${result.message || 'Respuesta inválida'}`, 'error');
        }
    } catch (error) {
        setStatus(`Error generando trama: ${error.message}`, 'error');
    }
}

async function suggestCatalogElements() {
    if (!state.selectedAgentId) {
        setStatus('Selecciona un agente primero.', 'error');
        return;
    }
    
    if (!state.selectedCatalog) {
        setStatus('Selecciona un catálogo primero.', 'error');
        return;
    }
    
    setStatus('Pidiendo sugerencias de elementos...', 'info');
    
    try {
        // Obtener los elementos del catálogo seleccionado
        let catalogElements = [];
        if (state.selectedCatalog === 'generic') {
            catalogElements = state.catalogs.generic || [];
        } else if (state.selectedCatalog.startsWith('generated_')) {
            const index = parseInt(state.selectedCatalog.replace('generated_', ''));
            catalogElements = state.catalogs.generated[index]?.elements || [];
        }
        
        // Llamar a la IA para sugerir elementos
        const response = await fetch('http://127.0.0.1:8000/enviar_mensaje', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                agente: state.selectedAgentName || 'ContextoHistorico',
                mensaje: `Para el proyecto: ${JSON.stringify(state.project.projectMeta, null, 2)}. 

Sugiere story elements adecuados del siguiente catálogo: ${JSON.stringify(catalogElements.slice(0, 30), null, 2)}. 

Devuelve SOLO JSON con un array de elementos recomendados: [{"id": "...", "name": "...", "type": "...", "reason": "..."}]`
            })
        });
        
        if (!response.ok) {
            throw new Error('Error al sugerir elementos');
        }
        
        const result = await response.json();
        
        if (result.status === 'success') {
            // Parsear la respuesta
            let suggestions;
            try {
                suggestions = typeof result.text === 'string' ? JSON.parse(result.text) : result.text;
            } catch (e) {
                setStatus(`Error parseando respuesta: ${e.message}`, 'error');
                return;
            }
            
            // Mostrar sugerencias (podríamos añadir un modal o panel)
            setStatus(`Sugerencias: ${suggestions.length} elementos recomendados.`, 'success');
            
            // Opcional: Resaltar los elementos sugeridos
            if (Array.isArray(suggestions)) {
                state.highlightedStoryElements = suggestions.map(s => s.id);
                renderCatalogStoryElements();
            }
        } else {
            setStatus(`Error: ${result.message || 'Respuesta inválida'}`, 'error');
        }
    } catch (error) {
        setStatus(`Error sugiriendo elementos: ${error.message}`, 'error');
    }
}

async function validatePlotWithCatalog() {
    if (!state.selectedAgentId) {
        setStatus('Selecciona un agente primero.', 'error');
        return;
    }
    
    if (!state.selectedCatalog) {
        setStatus('Selecciona un catálogo primero.', 'error');
        return;
    }
    
    // Obtener la trama activa
    const activePlot = state.project.plots.find(p => p.id === state.activePlotId);
    if (!activePlot) {
        setStatus('Selecciona una trama primero.', 'error');
        return;
    }
    
    setStatus('Validando trama con catálogo...', 'info');
    
    try {
        // Obtener los elementos del catálogo seleccionado
        let catalogElements = [];
        if (state.selectedCatalog === 'generic') {
            catalogElements = state.catalogs.generic || [];
        } else if (state.selectedCatalog.startsWith('generated_')) {
            const index = parseInt(state.selectedCatalog.replace('generated_', ''));
            catalogElements = state.catalogs.generated[index]?.elements || [];
        }
        
        // Llamar a la IA para validar
        const response = await fetch('http://127.0.0.1:8000/enviar_mensaje', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                agente: state.selectedAgentName || 'Verificacion',
                mensaje: `Valida la siguiente trama contra el catálogo: 

Trama: ${JSON.stringify(activePlot, null, 2)} 

Catálogo: ${JSON.stringify(catalogElements.slice(0, 20), null, 2)}. 

Devuelve SOLO JSON con el formato: {"valid": true/false, "issues": [...], "suggestions": [...]}`
            })
        });
        
        if (!response.ok) {
            throw new Error('Error al validar');
        }
        
        const result = await response.json();
        
        if (result.status === 'success') {
            // Parsear la respuesta
            let validation;
            try {
                validation = typeof result.text === 'string' ? JSON.parse(result.text) : result.text;
            } catch (e) {
                setStatus(`Error parseando respuesta: ${e.message}`, 'error');
                return;
            }
            
            if (validation.valid) {
                setStatus('Trama válida según el catálogo.', 'success');
            } else {
                setStatus(`Trama con issues: ${validation.issues?.join(', ') || 'Desconocido'}`, 'warning');
            }
        } else {
            setStatus(`Error: ${result.message || 'Respuesta inválida'}`, 'error');
        }
    } catch (error) {
        setStatus(`Error validando: ${error.message}`, 'error');
    }
}

async function loadProject() {
  clearStatus();
  const raw = await apiGet('/api/project');
  state.project = ensureProjectShape(raw);

  if (state.project.narratives?.length) {
    state.activeNarrativeId = state.project.narratives[0].id;
  } else {
    state.activeNarrativeId = null;
  }

  if (state.project.projectCharacters?.length) {
    state.activeCharacterId = state.project.projectCharacters[0].id;
  } else {
    state.activeCharacterId = null;
  }

  if (state.project.plots?.length) {
    state.activePlotId = state.project.plots[0].id;
  } else {
    state.activePlotId = null;
  }

  renderAll();
  addActivity({
    label: 'Proyecto cargado',
    detail: state.project.projectMeta?.title || 'Sin título'
  });
  setStatus('Proyecto cargado correctamente.', 'success');
}

async function saveProject() {
  syncProjectFromForms();
  await apiPost('/api/project', state.project);
  addActivity({
    label: 'Proyecto guardado',
    detail: state.project.projectMeta?.title || 'Sin título'
  });
  setStatus('Proyecto guardado correctamente.', 'success');
}

function renderAll() {
  renderProjectMeta();
  renderProjectForm();
  renderNarrativeList();
  renderNarrativeEditor();
  
  renderCharacterList();
  renderCharacterEditor();
  renderPlotList();
  renderPlotEditor();
  renderEvents();
  renderReview();
  renderRawJson();
}

function renderProjectMeta() {
    const m = state.project.projectMeta || {};
    const el = id => document.getElementById(id);
    if (el("metaProjectTitle")) el("metaProjectTitle").textContent = m.title || "-";
    if (el("metaNarrativeCount")) el("metaNarrativeCount").textContent = String((state.project.narratives || []).length);
    if (el("metaCharacterCount")) el("metaCharacterCount").textContent = String((state.project.projectCharacters || []).length);
    if (el("metaPlotCount")) el("metaPlotCount").textContent = String((state.project.plots || []).length);
    if (el("metaEventCount")) el("metaEventCount").textContent = String((state.project.events || []).length);
    if (el("metaStatus")) el("metaStatus").textContent = "Sincronizado";
}

function syncProjectMetaFromForm() {
    if (!state.project.projectMeta) state.project.projectMeta = {};
    const m = state.project.projectMeta;
    const el = id => document.getElementById(id);
    if (el("projectMetaTitle")) m.title = el("projectMetaTitle").value.trim();
    if (el("projectMetaFormat")) m.format = el("projectMetaFormat").value;
    if (el("projectMetaSummary")) m.summary = el("projectMetaSummary").value.trim();
    if (el("projectMetaWorldContext")) m.worldContext = el("projectMetaWorldContext").value.trim();
    if (el("projectMetaAllowedGenres")) m.allowedGenres = textToLines(el("projectMetaAllowedGenres").value);
    if (!m.contentLimits) m.contentLimits = {};
    if (el("projectMetaMaxRating")) m.contentLimits.maxRating = el("projectMetaMaxRating").value;
    if (el("projectMetaBlockedTags")) m.contentLimits.blockedSensitivityTags = textToLines(el("projectMetaBlockedTags").value);
    if (el("projectMetaNotes")) m.notes = el("projectMetaNotes").value.trim();
}

function renderNarrativeForm() {
    const p = state.project.project;
    $("#projectTitle").value = p.title || "";
    $("#projectAct").value = p.act || "Act1";
    $("#projectTone").value = p.tone || "Ambiguous";
    $("#projectHistoricalScope").value = p.historical_scope || "PlausibleInferred";
    $("#projectSummary").value = p.summary || "";
    $("#projectSpaces").value = linesToText(p.spaces);
    $("#projectFactions").value = linesToText(p.factions);
    $("#projectStakes").value = linesToText(p.stakes);
    $("#projectTags").value = linesToText(p.tags);
}

function syncNarrativeFromForm() {
  const n = (state.project.narratives || []).find(x => x.id === state.activeNarrativeId);
  if (!n) return;
  if ($("#projectTitle")) n.title = $("#projectTitle").value.trim();
  if ($("#projectAct")) n.act = $("#projectAct").value;
  if ($("#projectTone")) n.tone = $("#projectTone").value;
  if ($("#projectHistoricalScope")) n.historicalscope = $("#projectHistoricalScope").value;
  if ($("#projectSummary")) n.summary = $("#projectSummary").value.trim();
  if ($("#projectSpaces")) n.spaces = textToLines($("#projectSpaces").value);
  if ($("#projectFactions")) n.factions = textToLines($("#projectFactions").value);
  if ($("#projectStakes")) n.stakes = textToLines($("#projectStakes").value);
  if ($("#projectTags")) n.tags = textToLines($("#projectTags").value);
}

function syncReviewFromForm() {
    state.project.review.summary = $("#reviewSummary").value.trim();
    state.project.review.issues = textToLines($("#reviewIssues").value);
}

function renderReview() {
    $("#reviewSummary").value = state.project.review?.summary || "";
    $("#reviewIssues").value = linesToText(state.project.review?.issues || []);
}

function syncProjectFromForms() {
  syncProjectMetaFromForm();
  syncActiveNarrativeFromForm();
  syncStoryElementsFromDom();
  syncActiveCharacterFromForm();
  syncActivePlotFromForm();
  syncEventsFromDom();
  syncReviewFromForm();
  renderRawJson();
  renderProjectMeta();
}

function renderNarrativeList() {
    const container = document.getElementById("narrativeList");
    if (!container) return;
    const narratives = state.project.narratives || [];
    if (!narratives.length) {
        container.className = "card-list empty-state";
        container.innerHTML = "No hay narrativas.";
        return;
    }
    container.className = "card-list";
    container.innerHTML = narratives.map(n => `
        <div class="entity-card-mini ${state.activeNarrativeId === n.id ? "is-active" : ""}" data-narrative-id="${escapeHtml(n.id)}" style="cursor:pointer;padding:10px 14px;margin-bottom:8px;border-radius:8px;border:1px solid var(--border)">
          <strong>${escapeHtml(n.title || "Sin título")}</strong>
          <span style="font-size:0.8rem;color:var(--text-muted)"> · ${escapeHtml(n.act || "")}</span>
        </div>
    `).join("");
    container.querySelectorAll("[data-narrative-id]").forEach(el => {
        el.addEventListener("click", () => {
            state.activeNarrativeId = el.dataset.narrativeId;
            renderNarrativeList();
            renderNarrativeEditor();
        });
    });
}

function renderNarrativeEditor() {
    const n = (state.project.narratives || []).find(x => x.id === state.activeNarrativeId);
    const el = id => document.getElementById(id);
    if (!n) {
        if (el("narrativeEditorTitle")) el("narrativeEditorTitle").textContent = "Editar narrativa";
        return;
    }
    if (el("narrativeEditorTitle")) el("narrativeEditorTitle").textContent = n.title || "Sin título";
    if (el("narTitle")) el("narTitle").value = n.title || "";
    if (el("narAct")) el("narAct").value = n.act || "Act1";
    if (el("narTone")) el("narTone").value = n.tone || "Ambiguous";
    if (el("narHistoricalScope")) el("narHistoricalScope").value = n.historicalscope || "PlausibleInferred";
    if (el("narSummary")) el("narSummary").value = n.summary || "";
    if (el("narDescription")) el("narDescription").value = n.description || "";
    if (el("narSpaces")) el("narSpaces").value = linesToText(n.spaces);
    if (el("narFactions")) el("narFactions").value = linesToText(n.factions);
    if (el("narStakes")) el("narStakes").value = linesToText(n.stakes);
    if (el("narTags")) el("narTags").value = linesToText(n.tags);
    if (el("narExtendedNotes")) el("narExtendedNotes").value = n.extendedNotes || "";
}

function syncActiveNarrativeFromForm() {
    const n = (state.project.narratives || []).find(x => x.id === state.activeNarrativeId);
    if (!n) return;
    const el = id => document.getElementById(id);
    if (el("narTitle")) n.title = el("narTitle").value.trim();
    if (el("narAct")) n.act = el("narAct").value;
    if (el("narTone")) n.tone = el("narTone").value;
    if (el("narHistoricalScope")) n.historicalscope = el("narHistoricalScope").value;
    if (el("narSummary")) n.summary = el("narSummary").value.trim();
    if (el("narDescription")) n.description = el("narDescription").value.trim();
    if (el("narSpaces")) n.spaces = textToLines(el("narSpaces").value);
    if (el("narFactions")) n.factions = textToLines(el("narFactions").value);
    if (el("narStakes")) n.stakes = textToLines(el("narStakes").value);
    if (el("narTags")) n.tags = textToLines(el("narTags").value);
    if (el("narExtendedNotes")) n.extendedNotes = el("narExtendedNotes").value.trim();
}

function renderProjectForm() {
    const m = state.project.projectMeta || {};
    const el = id => document.getElementById(id);
    if (el("projectMetaTitle")) el("projectMetaTitle").value = m.title || "";
    if (el("projectMetaFormat")) el("projectMetaFormat").value = m.format || "theatre_play";
    if (el("projectMetaSummary")) el("projectMetaSummary").value = m.summary || "";
    if (el("projectMetaWorldContext")) el("projectMetaWorldContext").value = m.worldContext || "";
    if (el("projectMetaAllowedGenres")) el("projectMetaAllowedGenres").value = linesToText(m.allowedGenres);
    if (el("projectMetaMaxRating")) el("projectMetaMaxRating").value = m.contentLimits?.maxRating || "PG-13";
    if (el("projectMetaBlockedTags")) el("projectMetaBlockedTags").value = linesToText(m.contentLimits?.blockedSensitivityTags);
    if (el("projectMetaNotes")) el("projectMetaNotes").value = m.notes || "";
}

function addNarrative() {
    const id = "nar_" + Date.now();
    state.project.narratives = state.project.narratives || [];
    state.project.narratives.push({
        id, title: "Nueva narrativa", summary: "", description: "",
        act: "Act1", tone: "Ambiguous", historicalscope: "PlausibleInferred",
        spaces: [], factions: [], stakes: [], tags: [],
        extendedNotes: "", futureCompat: {},
        cast: { rules: { protagonistMode: "single", maxAntagonists: 2, maxSupporting: 4 }, entries: [] }
    });
    state.activeNarrativeId = id;
    renderNarrativeList();
    renderNarrativeEditor();
    renderProjectMeta();
    renderRawJson();
}

function renderCharacterList() {
    const container = document.getElementById("characterList");
    if (!container) return;
    const chars = state.project.projectCharacters || [];
    if (!chars.length) {
        container.className = "card-list empty-state";
        container.innerHTML = "No hay personajes.";
        return;
    }
    container.className = "card-list";
    container.innerHTML = chars.map(c => `
        <div class="entity-card-mini ${state.activeCharacterId === c.id ? "is-active" : ""}" data-char-id="${escapeHtml(c.id)}" style="cursor:pointer;padding:10px 14px;margin-bottom:8px;border-radius:8px;border:1px solid var(--border)">
          <strong>${escapeHtml(c.name || "Sin nombre")}</strong>
          <span style="font-size:0.8rem;color:var(--text-muted)"> · ${escapeHtml(c.projectRoleType || "")}</span>
        </div>
    `).join("");
    container.querySelectorAll("[data-char-id]").forEach(el => {
        el.addEventListener("click", () => {
            state.activeCharacterId = el.dataset.charId;
            renderCharacterList();
            renderCharacterEditor();
        });
    });
}

function renderCharacterEditor() {
    const c = (state.project.projectCharacters || []).find(x => x.id === state.activeCharacterId);
    const el = id => document.getElementById(id);
    if (!c) return;
    if (el("characterEditorTitle")) el("characterEditorTitle").textContent = c.name || "Sin nombre";
    if (el("charName")) el("charName").value = c.name || "";
    if (el("charProjectRoleType")) el("charProjectRoleType").value = c.projectRoleType || "flex";
    if (el("charSummary")) el("charSummary").value = c.summary || "";
    if (el("charDescription")) el("charDescription").value = c.description || "";
    const np = c.narrativeProfile || {};
    if (el("charDramaticFunctions")) el("charDramaticFunctions").value = linesToText(np.dramaticFunctions);
    if (el("charCoreDrives")) el("charCoreDrives").value = linesToText(np.coreDrives);
    if (el("charTraits")) el("charTraits").value = linesToText(np.traits);
    if (el("charToneFit")) el("charToneFit").value = linesToText(np.toneFit);
    const wf = c.worldFit || {};
    if (el("charAllowedPeriods")) el("charAllowedPeriods").value = linesToText(wf.allowedPeriods);
    if (el("charPreferredSettings")) el("charPreferredSettings").value = linesToText(wf.preferredSettings);
    if (el("charExtendedNotes")) el("charExtendedNotes").value = c.extendedNotes || "";
}

function syncActiveCharacterFromForm() {
    const c = (state.project.projectCharacters || []).find(x => x.id === state.activeCharacterId);
    if (!c) return;
    const el = id => document.getElementById(id);
    if (el("charName")) c.name = el("charName").value.trim();
    if (el("charProjectRoleType")) c.projectRoleType = el("charProjectRoleType").value;
    if (el("charSummary")) c.summary = el("charSummary").value.trim();
    if (el("charDescription")) c.description = el("charDescription").value.trim();
    c.narrativeProfile = c.narrativeProfile || {};
    if (el("charDramaticFunctions")) c.narrativeProfile.dramaticFunctions = textToLines(el("charDramaticFunctions").value);
    if (el("charCoreDrives")) c.narrativeProfile.coreDrives = textToLines(el("charCoreDrives").value);
    if (el("charTraits")) c.narrativeProfile.traits = textToLines(el("charTraits").value);
    if (el("charToneFit")) c.narrativeProfile.toneFit = textToLines(el("charToneFit").value);
    c.worldFit = c.worldFit || {};
    if (el("charAllowedPeriods")) c.worldFit.allowedPeriods = textToLines(el("charAllowedPeriods").value);
    if (el("charPreferredSettings")) c.worldFit.preferredSettings = textToLines(el("charPreferredSettings").value);
    if (el("charExtendedNotes")) c.extendedNotes = el("charExtendedNotes").value.trim();
}

function addCharacter() {
    const id = "char_" + Date.now();
    state.project.projectCharacters = state.project.projectCharacters || [];
    state.project.projectCharacters.push({
        id, name: "Nuevo personaje", projectRoleType: "flex",
        summary: "", description: "", extendedNotes: "", futureCompat: {},
        narrativeProfile: { dramaticFunctions: [], coreDrives: [], traits: [], toneFit: [] },
        worldFit: { allowedPeriods: [], preferredSettings: [], genreAffinity: [] },
        relationships: []
    });
    state.activeCharacterId = id;
    renderCharacterList();
    renderCharacterEditor();
    renderProjectMeta();
    renderRawJson();
}

function renderPlotList() {
    const container = document.getElementById("plotList");
    if (!container) return;
    const plots = state.project.plots || [];
    if (!plots.length) {
        container.className = "card-list empty-state";
        container.innerHTML = "No hay tramas.";
        return;
    }
    container.className = "card-list";
    container.innerHTML = plots.map(pl => `
        <div class="entity-card-mini ${state.activePlotId === pl.id ? "is-active" : ""}" data-plot-id="${escapeHtml(pl.id)}" style="cursor:pointer;padding:10px 14px;margin-bottom:8px;border-radius:8px;border:1px solid var(--border)">
          <strong>${escapeHtml(pl.title || "Sin título")}</strong>
          <span style="font-size:0.8rem;color:var(--text-muted)"> · ${escapeHtml(pl.status || "draft")}</span>
        </div>
    `).join("");
    container.querySelectorAll("[data-plot-id]").forEach(el => {
        el.addEventListener("click", () => {
            state.activePlotId = el.dataset.plotId;
            renderPlotList();
            renderPlotEditor();
        });
    });
}

function renderPlotEditor() {
    const pl = (state.project.plots || []).find(x => x.id === state.activePlotId);
    const el = id => document.getElementById(id);
    if (!pl) return;
    if (el("plotEditorTitle")) el("plotEditorTitle").textContent = pl.title || "Sin título";
    if (el("plotTitle")) el("plotTitle").value = pl.title || "";
    if (el("plotStatus")) el("plotStatus").value = pl.status || "draft";
    if (el("plotSummary")) el("plotSummary").value = pl.summary || "";
    if (el("plotNarrativeLink")) {
        const opts = ["<option value=\">Selecciona narrativa</option>"].concat(
            (state.project.narratives || []).map(n => `
                <option value="${escapeHtml(n.id)}" ${pl.narrativeId === n.id ? "selected" : ""}>${escapeHtml(n.title || n.id)}</option>`
            )
        );
        el("plotNarrativeLink").innerHTML = opts.join("");
    }
    const cs = pl.characterSelection || {};
    if (el("plotProtagonistId")) {
        const chars = state.project.projectCharacters || [];
        const opts = ["<option value=\">Sin protagonista</option>"].concat(
            chars.map(c => `
                <option value="${escapeHtml(c.id)}" ${cs.protagonistCharacterId === c.id ? "selected" : ""}>${escapeHtml(c.name || c.id)}</option>`
            )
        );
        el("plotProtagonistId").innerHTML = opts.join("");
    }
    if (el("plotAntagonistIds")) el("plotAntagonistIds").value = linesToText(cs.antagonistCharacterIds);
    if (el("plotSupportingIds")) el("plotSupportingIds").value = linesToText(cs.supportingCharacterIds);
    const sel = pl.selection || {};
    if (el("plotThemeIds")) el("plotThemeIds").value = linesToText(sel.themeIds);
    if (el("plotEventIds")) el("plotEventIds").value = linesToText(sel.eventIds);
    if (el("plotSettingIds")) el("plotSettingIds").value = linesToText(sel.settingIds);
    if (el("plotFinaleId")) el("plotFinaleId").value = sel.finaleId || "";
    if (el("plotExtendedNotes")) el("plotExtendedNotes").value = pl.extendedNotes || "";
}

function syncActivePlotFromForm() {
    const pl = (state.project.plots || []).find(x => x.id === state.activePlotId);
    if (!pl) return;
    const el = id => document.getElementById(id);
    if (el("plotTitle")) pl.title = el("plotTitle").value.trim();
    if (el("plotStatus")) pl.status = el("plotStatus").value;
    if (el("plotSummary")) pl.summary = el("plotSummary").value.trim();
    if (el("plotNarrativeLink")) pl.narrativeId = el("plotNarrativeLink").value;
    pl.characterSelection = pl.characterSelection || {};
    if (el("plotProtagonistId")) pl.characterSelection.protagonistCharacterId = el("plotProtagonistId").value || null;
    if (el("plotAntagonistIds")) pl.characterSelection.antagonistCharacterIds = textToLines(el("plotAntagonistIds").value);
    if (el("plotSupportingIds")) pl.characterSelection.supportingCharacterIds = textToLines(el("plotSupportingIds").value);
    pl.selection = pl.selection || {};
    if (el("plotThemeIds")) pl.selection.themeIds = textToLines(el("plotThemeIds").value);
    if (el("plotEventIds")) pl.selection.eventIds = textToLines(el("plotEventIds").value);
    if (el("plotSettingIds")) pl.selection.settingIds = textToLines(el("plotSettingIds").value);
    if (el("plotFinaleId")) pl.selection.finaleId = el("plotFinaleId").value.trim() || null;
    if (el("plotExtendedNotes")) pl.extendedNotes = el("plotExtendedNotes").value.trim();
}

function addPlot() {
    const id = "plot_" + Date.now();
    state.project.plots = state.project.plots || [];
    state.project.plots.push({
        id, title: "Nueva trama", narrativeId: state.activeNarrativeId || "",
        summary: "", status: "draft", extendedNotes: "", futureCompat: {},
        characterSelection: { protagonistCharacterId: null, antagonistCharacterIds: [], supportingCharacterIds: [], entries: [] },
        selection: { themeIds: [], eventIds: [], settingIds: [], finaleId: null, genreIds: [] }
    });
    state.activePlotId = id;
    renderPlotList();
    renderPlotEditor();
    renderProjectMeta();
    renderRawJson();
}

function renderRawJson() {
    $("#rawJsonView").value = JSON.stringify(state.project, null, 2);
}

function parseChoicesText(text) {
    const trimmed = String(text || "").trim();
    if (!trimmed) return [];
    const parsed = safeJsonParse(trimmed, null);
    return Array.isArray(parsed) ? parsed : [];
}

function choicesToText(choices) {
    return JSON.stringify(Array.isArray(choices) ? choices : [], null, 2);
}

function createEmptyStoryElement(type = "Theme") {
  const activeNarrative = (state.project.narratives || []).find(n => n.id === state.activeNarrativeId);
  const defaultTone = activeNarrative?.tone || state.project.projectMeta?.toneProfile?.tone || "Ambiguous";
  return {
    id: generateId("story"),
    type,
    label: "",
    description: "",
    tone: defaultTone,
    selected: false
  };
}

function createEmptyEvent(storyElementId = "") {
    return {
        id: generateId("event"),
        label: "",
        title: "",
        story_element_id: storyElementId,
        body_text: "",
        flavor_text: "",
        choices: [],
        consequences: [],
        assets: []
    };
}



function syncStoryElementsFromDom() {
    const cards = $all(".story-card");
    const domMap = new Map();

    cards.forEach((card) => {
        const oldId = card.dataset.storyId;
        const story = {
            id: card.querySelector(".story-id-input").value.trim() || generateId("story"),
            label: card.querySelector(".story-label-input").value.trim(),
            type: card.querySelector(".story-type-input").value,
            tone: card.querySelector(".story-tone-input").value.trim(),
            description: card.querySelector(".story-description-input").value.trim(),
            selected: card.querySelector(".story-selected-input").checked
        };
        domMap.set(oldId, story);
    });

    state.project.storyelements = state.project.storyelements
        .map((item) => domMap.get(item.id) || item);

    const filteredIds = new Set(getFilteredStoryElements().map((item) => item.id));
    cards.forEach((card) => {
        const oldId = card.dataset.storyId;
        const updated = domMap.get(oldId);
        if (updated) {
            card.dataset.storyId = updated.id;
        }
    });

    renderProjectMeta();
    renderRawJson();
    
}

function addStoryElement(type = "Theme") {
    syncProjectFromForms();
    state.project.storyelements.push(createEmptyStoryElement(type));
    
    
    setStatus("Story element añadido.", "success");
}

function renderEvents() {
    const container = $("#eventsList");
    if (!state.project.events.length) {
        container.className = "card-list empty-state";
        container.innerHTML = "No hay eventos todavía.";
        
        renderProjectMeta();
        renderRawJson();
        return;
    }

    container.className = "card-list";
    container.innerHTML = "";

    state.project.events.forEach((event) => {
        const template = $("#eventCardTemplate");
        const fragment = template.content.cloneNode(true);
        const card = fragment.querySelector(".event-card");
        card.dataset.eventId = event.id;

        const idInput = fragment.querySelector(".event-id-input");
        const titleInput = fragment.querySelector(".event-title-input");
        const labelInput = fragment.querySelector(".event-label-input");
        const storySelect = fragment.querySelector(".event-story-element-id-input");
        const bodyInput = fragment.querySelector(".event-body-text-input");
        const flavorInput = fragment.querySelector(".event-flavor-text-input");
        const choicesInput = fragment.querySelector(".event-choices-input");
        const consequencesInput = fragment.querySelector(".event-consequences-input");
        const assetsInput = fragment.querySelector(".event-assets-input");
        const removeBtn = fragment.querySelector(".event-remove-btn");

        idInput.value = event.id || "";
        titleInput.value = event.title || "";
        labelInput.value = event.label || "";
        populateStorySelect(storySelect, event.story_element_id || "");
        bodyInput.value = event.body_text || "";
        flavorInput.value = event.flavor_text || "";
        choicesInput.value = choicesToText(event.choices || []);
        consequencesInput.value = linesToText(event.consequences || []);
        assetsInput.value = linesToText(event.assets || []);

        [
            idInput,
            titleInput,
            labelInput,
            storySelect,
            bodyInput,
            flavorInput,
            choicesInput,
            consequencesInput,
            assetsInput
        ].forEach((input) => {
            const eventName = input.tagName === "SELECT" ? "change" : "input";
            input.addEventListener(eventName, syncEventsFromDom);
        });

        removeBtn.addEventListener("click", () => {
            state.project.events = state.project.events.filter((e) => e.id !== event.id);
            renderEvents();
            renderProjectMeta();
            renderRawJson();
            setStatus("Evento eliminado.", "success");
        });

        container.appendChild(fragment);
    });

    
    renderProjectMeta();
    renderRawJson();
}

function syncEventsFromDom() {
    const cards = $all(".event-card");
    const domMap = new Map();

    cards.forEach((card) => {
        const oldId = card.dataset.eventId;
        const event = {
            id: card.querySelector(".event-id-input").value.trim() || generateId("event"),
            title: card.querySelector(".event-title-input").value.trim(),
            label: card.querySelector(".event-label-input").value.trim(),
            story_element_id: card.querySelector(".event-story-element-id-input").value,
            body_text: card.querySelector(".event-body-text-input").value.trim(),
            flavor_text: card.querySelector(".event-flavor-text-input").value.trim(),
            choices: parseChoicesText(card.querySelector(".event-choices-input").value),
            consequences: textToLines(card.querySelector(".event-consequences-input").value),
            assets: textToLines(card.querySelector(".event-assets-input").value)
        };
        domMap.set(oldId, event);
    });

    state.project.events = state.project.events.map((event) => domMap.get(event.id) || event);

    cards.forEach((card) => {
        const oldId = card.dataset.eventId;
        const updated = domMap.get(oldId);
        if (updated) {
            card.dataset.eventId = updated.id;
        }
    });

    renderProjectMeta();
    renderRawJson();
}

function addEvent(storyElementId = "") {
    syncProjectFromForms();
    state.project.events.push(createEmptyEvent(storyElementId));
    renderEvents();
    setStatus("Evento añadido.", "success");
}

function populateStorySelect(selectEl, selectedId = "") {
    const options = [`<option value="">Selecciona story element</option>`].concat(
        state.project.storyelements.map((item) => `
            <option value="${escapeHtml(item.id || "")}" ${selectedId === item.id ? "selected" : ""}>
                ${escapeHtml(item.label || item.id || "(sin label)")} · ${escapeHtml(item.type || "")}
            </option>
        `)
    );
    selectEl.innerHTML = options.join("");
}



function applyNarrativeData(data) {
  if (!data || typeof data !== 'object') return;

  const targetId = state.activeNarrativeId || state.project.narratives?.[0]?.id || 'nar_001';
  let target = (state.project.narratives || []).find(n => n.id === targetId);

  if (!target) {
    target = {
      id: targetId,
      title: 'Nueva narrativa',
      summary: '',
      description: '',
      act: 'Act1',
      tone: 'Ambiguous',
      historicalscope: 'PlausibleInferred',
      spaces: [],
      factions: [],
      stakes: [],
      tags: [],
      extendedNotes: '',
      futureCompat: {},
      cast: { rules: { protagonistMode: 'single', maxAntagonists: 2, maxSupporting: 4 }, entries: [] }
    };
    state.project.narratives.push(target);
    state.activeNarrativeId = target.id;
  }

  target.title = data.title ?? target.title;
  target.summary = data.summary ?? target.summary;
  target.act = data.act ?? target.act;
  target.tone = data.tone ?? target.tone;
  target.historicalscope = data.historicalscope ?? target.historicalscope;
  target.spaces = Array.isArray(data.spaces) ? data.spaces : target.spaces;
  target.factions = Array.isArray(data.factions) ? data.factions : target.factions;
  target.stakes = Array.isArray(data.stakes) ? data.stakes : target.stakes;
  target.tags = Array.isArray(data.tags) ? data.tags : target.tags;
}

function normalizeStoryElement(item) {
    const activeNarrative = (state.project.narratives || []).find(n => n.id === state.activeNarrativeId);
    const defaultTone = activeNarrative?.tone || state.project.projectMeta?.toneProfile?.tone || "Ambiguous";
    return {
      id: item.id || generateId("story"),
      type: item.type || "Theme",
      label: item.label || "",
      description: item.description || "",
      tone: item.tone || defaultTone,
      selected: false
    };
}

function applyStoryElementsData(data) {
    if (!Array.isArray(data)) return;
    state.project.storyelements = data.map(normalizeStoryElement);
}

function normalizeEventData(data) {
    return {
        id: data.id || generateId("event"),
        label: data.label || "",
        title: data.title || "",
        story_element_id: data.story_element_id || "",
        body_text: data.body_text || "",
        flavor_text: data.flavor_text || "",
        choices: Array.isArray(data.choices) ? data.choices : [],
        consequences: Array.isArray(data.consequences) ? data.consequences : [],
        assets: Array.isArray(data.assets) ? data.assets : []
    };
}

function applyEventData(data) {
    if (!data || typeof data !== "object") return;
    const normalized = normalizeEventData(data);
    const existingIndex = state.project.events.findIndex((event) => event.id === normalized.id);

    if (existingIndex >= 0) {
        state.project.events[existingIndex] = normalized;
    } else {
        state.project.events.push(normalized);
    }
}

function applyReviewData(data) {
    if (!data || typeof data !== "object") return;
    state.project.review.summary = data.summary || "";
    state.project.review.issues = Array.isArray(data.issues) ? data.issues : [];
}

function appendAiHistory(response) {
  if (!Array.isArray(state.project.aihistory)) state.project.aihistory = [];
  state.project.aihistory.push({
    timestamp: new Date().toISOString(),
    response
  });
  if (state.project.aihistory.length > 50) {
    state.project.aihistory = state.project.aihistory.slice(-50);
  }
}

function applyProjectData(data) {
  if (!data || typeof data !== 'object') return;
  state.project.projectMeta = {
    ...(state.project.projectMeta || {}),
    ...data,
    toneProfile: {
      ...(state.project.projectMeta?.toneProfile || {}),
      ...(data.toneProfile || {})
    },
    contentLimits: {
      ...(state.project.projectMeta?.contentLimits || {}),
      ...(data.contentLimits || {})
    },
    productionConstraints: {
      ...(state.project.projectMeta?.productionConstraints || {}),
      ...(data.productionConstraints || {})
    }
  };
}

function applyCharactersData(data) {
  if (!Array.isArray(data)) return;
  state.project.projectCharacters = data.map((c, idx) => ({
    id: c.id || `char_${Date.now()}_${idx}`,
    name: c.name || 'Sin nombre',
    projectRoleType: c.projectRoleType || 'flex',
    summary: c.summary || '',
    description: c.description || '',
    extendedNotes: c.extendedNotes || '',
    futureCompat: c.futureCompat || {},
    narrativeProfile: {
      dramaticFunctions: Array.isArray(c.narrativeProfile?.dramaticFunctions) ? c.narrativeProfile.dramaticFunctions : [],
      coreDrives: Array.isArray(c.narrativeProfile?.coreDrives) ? c.narrativeProfile.coreDrives : [],
      traits: Array.isArray(c.narrativeProfile?.traits) ? c.narrativeProfile.traits : [],
      toneFit: Array.isArray(c.narrativeProfile?.toneFit) ? c.narrativeProfile.toneFit : []
    },
    worldFit: {
      allowedPeriods: Array.isArray(c.worldFit?.allowedPeriods) ? c.worldFit.allowedPeriods : [],
      preferredSettings: Array.isArray(c.worldFit?.preferredSettings) ? c.worldFit.preferredSettings : [],
      genreAffinity: Array.isArray(c.worldFit?.genreAffinity) ? c.worldFit.genreAffinity : []
    },
    relationships: Array.isArray(c.relationships) ? c.relationships : []
  }));
  if (state.project.projectCharacters.length && !state.activeCharacterId) {
    state.activeCharacterId = state.project.projectCharacters[0].id;
  }
}

function applyPlotsData(data) {
  if (!Array.isArray(data)) return;
  state.project.plots = data.map((pl, idx) => ({
    id: pl.id || `plot_${Date.now()}_${idx}`,
    title: pl.title || 'Sin título',
    narrativeId: pl.narrativeId || (state.project.narratives[0]?.id || ''),
    summary: pl.summary || '',
    status: pl.status || 'draft',
    extendedNotes: pl.extendedNotes || '',
    futureCompat: pl.futureCompat || {},
    characterSelection: {
      protagonistCharacterId: pl.characterSelection?.protagonistCharacterId || null,
      antagonistCharacterIds: Array.isArray(pl.characterSelection?.antagonistCharacterIds) ? pl.characterSelection.antagonistCharacterIds : [],
      supportingCharacterIds: Array.isArray(pl.characterSelection?.supportingCharacterIds) ? pl.characterSelection.supportingCharacterIds : [],
      entries: Array.isArray(pl.characterSelection?.entries) ? pl.characterSelection.entries : []
    },
    selection: {
      themeIds: Array.isArray(pl.selection?.themeIds) ? pl.selection.themeIds : [],
      eventIds: Array.isArray(pl.selection?.eventIds) ? pl.selection.eventIds : [],
      settingIds: Array.isArray(pl.selection?.settingIds) ? pl.selection.settingIds : [],
      finaleId: pl.selection?.finaleId || null,
      genreIds: Array.isArray(pl.selection?.genreIds) ? pl.selection.genreIds : []
    }
  }));
  if (state.project.plots.length && !state.activePlotId) {
    state.activePlotId = state.project.plots[0].id;
  }
}

function applyAiResult(section, response) {
  if (response.status !== 'success') {
    const message = response.message || 'La IA devolvió un error.';
    addActivity({ label: `IA ${section}`, detail: message });
    setStatus(message, 'error');
    return;
  }

  if (section === 'project') {
    applyProjectData(response.data);
  } else if (section === 'narrative') {
    applyNarrativeData(response.data);
  } else if (section === 'characters') {
    applyCharactersData(response.data);
  } else if (section === 'plots') {
    applyPlotsData(response.data);
  } else if (section === 'story-elements') {
    applyStoryElementsData(response.data);
  } else if (section === 'event') {
    applyEventData(response.data);
  } else if (section === 'review') {
    applyReviewData(response.data);
  }

  appendAiHistory(response);

  const agent = response.meta?.agent || 'IA';
  addActivity({
    label: `${section} · ${response.action || 'acción'}`,
    detail: `Agente: ${agent}`
  });

  renderAll();
  setStatus(`Respuesta IA aplicada en ${section}.`, 'success');
}

async function runAi(section, action) {
    syncProjectFromForms();
    clearStatus();

    const endpointMap = {
        "narrative": "/api/ai/narrative",
        "story-elements": "/api/ai/story-elements",
        "event": "/api/ai/event",
        "review": "/api/ai/review",
        "project": "/api/ai/project",
        "characters": "/api/ai/characters",
        "plots": "/api/ai/plots"
    };

    const endpoint = endpointMap[section];
    if (!endpoint) {
        setStatus(`No existe endpoint para la sección ${section}.`, "error");
        return;
    }

    // Validación para eventos
    if (section === 'event' && action === 'generate_from_story_element') {
        const selectedStoryId = document.getElementById('eventStoryElementLink')?.value;
        if (!selectedStoryId) {
            setStatus('Debes seleccionar un Story Element antes de generar un evento.', 'error');
            return;
        }
        
        // Verificar que el story element existe
        const storyElementExists = state.project.storyelements?.some(se => se.id === selectedStoryId);
        if (!storyElementExists) {
            setStatus('El Story Element seleccionado no existe en el proyecto.', 'error');
            return;
        }
    }

    setStatus(`Ejecutando IA: ${section} / ${action}...`, "info");

    try {
        let payloadProject = clone(state.project);

        if (section === 'event' && action === 'generate_from_story_element') {
            const selectedStoryId = document.getElementById('eventStoryElementLink')?.value;
            if (selectedStoryId) payloadProject.selectedstoryelementid = selectedStoryId;
        }
        
        // Para acciones de plots con catálogo, incluir el catálogo seleccionado
        if (section === 'plots' && (action.includes('catalog') || action.includes('generate_from_catalog'))) {
            payloadProject.selectedCatalog = state.selectedCatalog;
            payloadProject.catalogElements = state.selectedCatalog === 'generic' 
                ? state.catalogs.generic 
                : (state.catalogs.generated[parseInt(state.selectedCatalog.replace('generated_', ''))]?.elements || []);
            payloadProject.highlightedElements = state.highlightedStoryElements;
        }

        const response = await apiPost(endpoint, {
            action,
            project: payloadProject,
            agent_id: state.selectedAgentId,
            agent_name: state.selectedAgentName
        });

        applyAiResult(section, response);
    } catch (error) {
        addActivity({
            label: `IA ${section}`,
            detail: `Error: ${error.message}`
        });
        setStatus(`Error ejecutando IA: ${error.message}`, "error");
    }
}

function switchTab(tabId) {
    state.activeTab = tabId;

    $all(".tab").forEach((tab) => {
        tab.classList.toggle("is-active", tab.dataset.tab === tabId);
    });

    $all(".tab-panel").forEach((panel) => {
        panel.classList.toggle("is-active", panel.id === `tab-${tabId}`);
    });

    if (tabId === "raw-json") {
        syncProjectFromForms();
        renderRawJson();
    }
    
    if (tabId === "agents") {
        renderAgentsTab();
    }
}

function copyRawJson() {
    syncProjectFromForms();
    const text = JSON.stringify(state.project, null, 2);
    navigator.clipboard.writeText(text)
        .then(() => setStatus("JSON copiado al portapapeles.", "success"))
        .catch(() => setStatus("No se pudo copiar el JSON.", "error"));
}

function bindTabs() {
    const tabs = document.querySelectorAll('.tab');
    tabs.forEach((tab) => {
      if (!tab) return;
      tab.addEventListener('click', () => switchTab(tab.dataset.tab));
    });
}

function bindNarrativeForm() {
  const selectors = [
    '#narTitle',
    '#narSummary',
    '#narDescription',
    '#narAct',
    '#narTone',
    '#narHistoricalScope',
    '#narSpaces',
    '#narFactions',
    '#narStakes',
    '#narTags',
    '#narExtendedNotes'
  ];

  selectors.forEach((selector) => {
    const el = document.querySelector(selector);
    if (!el) {
      console.warn('[bindNarrativeForm] missing element:', selector);
      return;
    }

    const tag = (el.tagName || '').toUpperCase();
    const eventName = tag === 'SELECT' ? 'change' : 'input';

    el.addEventListener(eventName, () => {
      if (typeof syncActiveNarrativeFromForm === 'function') {
        syncActiveNarrativeFromForm();
      }
      if (typeof renderNarrativeList === 'function') {
        renderNarrativeList();
      }
      if (typeof renderRawJson === 'function') {
        renderRawJson();
      }
    });
  });

  const btnGenerate = document.getElementById('btnAiNarrativeGenerate');
  if (btnGenerate) {
    btnGenerate.addEventListener('click', () => runAi('narrative', 'generate'));
  } else {
    console.warn('[bindNarrativeForm] missing element: #btnAiNarrativeGenerate');
  }

  const btnRefine = document.getElementById('btnAiNarrativeRefine');
  if (btnRefine) {
    btnRefine.addEventListener('click', () => runAi('narrative', 'refine'));
  } else {
    console.warn('[bindNarrativeForm] missing element: #btnAiNarrativeRefine');
  }
}

function bindStoryElements() {
    const storyFilterType = document.getElementById('storyFilterType');
    if (storyFilterType) {
      storyFilterType.addEventListener('change', (event) => {
        state.storyFilterType = event.target.value;
        
      });
    } else {
      console.warn('[bindStoryElements] missing element: #storyFilterType');
    }

    const btnAddStoryElement = document.getElementById('btnAddStoryElement');
    if (btnAddStoryElement) {
      btnAddStoryElement.addEventListener('click', () => addStoryElement('Theme'));
    } else {
      console.warn('[bindStoryElements] missing element: #btnAddStoryElement');
    }

    const btnAddPresetStoryElement = document.getElementById('btnAddPresetStoryElement');
    const storyPresetType = document.getElementById('storyPresetType');
    if (btnAddPresetStoryElement && storyPresetType) {
      btnAddPresetStoryElement.addEventListener('click', () => {
        const type = storyPresetType.value || 'Theme';
        addStoryElement(type);
      });
    } else {
      if (!btnAddPresetStoryElement) console.warn('[bindStoryElements] missing element: #btnAddPresetStoryElement');
      if (!storyPresetType) console.warn('[bindStoryElements] missing element: #storyPresetType');
    }

    const btnAiStoryPropose = document.getElementById('btnAiStoryPropose');
    if (btnAiStoryPropose) {
      btnAiStoryPropose.addEventListener('click', () => runAi('story-elements', 'propose'));
    } else {
      console.warn('[bindStoryElements] missing element: #btnAiStoryPropose');
    }

    const btnAiStoryEnrichSelected = document.getElementById('btnAiStoryEnrichSelected');
    if (btnAiStoryEnrichSelected) {
      btnAiStoryEnrichSelected.addEventListener('click', () => {
        const hasSelected = state.project.storyelements.some((item) => item.selected);
        if (!hasSelected) {
          setStatus('Selecciona al menos un story element para enriquecer.', 'error');
          return;
        }
        runAi('story-elements', 'enrich_selected');
      });
    } else {
      console.warn('[bindStoryElements] missing element: #btnAiStoryEnrichSelected');
    }
}

function bindEvents() {
    const btnAddEvent = document.getElementById('btnAddEvent');
    const eventStoryElementLink = document.getElementById('eventStoryElementLink');
    if (btnAddEvent && eventStoryElementLink) {
      btnAddEvent.addEventListener('click', () => addEvent(eventStoryElementLink.value || ''));
    } else {
      if (!btnAddEvent) console.warn('[bindEvents] missing element: #btnAddEvent');
      if (!eventStoryElementLink) console.warn('[bindEvents] missing element: #eventStoryElementLink');
    }

    const btnAiGenerateEvent = document.getElementById('btnAiGenerateEvent');
    if (btnAiGenerateEvent) {
      btnAiGenerateEvent.addEventListener('click', () => runAi('event', 'generate_from_story_element'));
    } else {
      console.warn('[bindEvents] missing element: #btnAiGenerateEvent');
    }

    const btnAiGenerateFromSelectedStory = document.getElementById('btnAiGenerateFromSelectedStory');
    if (btnAiGenerateFromSelectedStory && eventStoryElementLink) {
      btnAiGenerateFromSelectedStory.addEventListener('click', () => {
        const storyId = eventStoryElementLink.value;
        if (!storyId) {
          setStatus('Selecciona un story element base antes de generar un evento.', 'error');
          return;
        }
        runAi('event', 'generate_from_story_element');
      });
    } else {
      if (!btnAiGenerateFromSelectedStory) console.warn('[bindEvents] missing element: #btnAiGenerateFromSelectedStory');
      if (!eventStoryElementLink) console.warn('[bindEvents] missing element: #eventStoryElementLink');
    }
}

function bindReview() {
    const reviewSummary = document.getElementById('reviewSummary');
    if (reviewSummary) {
      reviewSummary.addEventListener('input', () => {
        syncReviewFromForm();
        renderRawJson();
      });
    } else {
      console.warn('[bindReview] missing element: #reviewSummary');
    }

    const reviewIssues = document.getElementById('reviewIssues');
    if (reviewIssues) {
      reviewIssues.addEventListener('input', () => {
        syncReviewFromForm();
        renderRawJson();
      });
    } else {
      console.warn('[bindReview] missing element: #reviewIssues');
    }

    const btnAiRunReview = document.getElementById('btnAiRunReview');
    if (btnAiRunReview) {
      btnAiRunReview.addEventListener('click', () => runAi('review', 'review'));
    } else {
      console.warn('[bindReview] missing element: #btnAiRunReview');
    }

    const btnReviewProject = document.getElementById('btnReviewProject');
    if (btnReviewProject) {
      btnReviewProject.addEventListener('click', () => runAi('review', 'review'));
    } else {
      console.warn('[bindReview] missing element: #btnReviewProject');
    }
}

function bindAgents() {
    // Selector global de agente
    const globalAgentSelect = document.getElementById('selectedAgent');
    if (globalAgentSelect) {
        globalAgentSelect.addEventListener('change', (event) => {
            const selectedOption = event.target.options[event.target.selectedIndex];
            if (selectedOption) {
                state.selectedAgentId = event.target.value;
                state.selectedAgentName = selectedOption.getAttribute('data-name') || selectedOption.text;
                setStatus(`Agente seleccionado: ${state.selectedAgentName}`, 'success');
            }
        });
    }
    
    // Botones de la pestaña Agentes
    const btnRefreshAgents = document.getElementById('btnRefreshAgents');
    if (btnRefreshAgents) {
        btnRefreshAgents.addEventListener('click', loadAgents);
    }
    
    const btnAddAgent = document.getElementById('btnAddAgent');
    if (btnAddAgent) {
        btnAddAgent.addEventListener('click', () => {
            // Limpiar formulario para nuevo agente
            const agentName = document.getElementById('agentName');
            const agentId = document.getElementById('agentId');
            const agentConversationId = document.getElementById('agentConversationId');
            
            if (agentName) agentName.value = '';
            if (agentId) agentId.value = '';
            if (agentConversationId) agentConversationId.value = '';
            
            // Habilitar botones
            const btnSave = document.getElementById('btnSaveAgent');
            const btnDelete = document.getElementById('btnDeleteAgent');
            const btnReset = document.getElementById('btnResetConversation');
            
            if (btnSave) btnSave.disabled = false;
            if (btnDelete) btnDelete.disabled = true;
            if (btnReset) btnReset.disabled = true;
            
            setStatus('Listo para añadir un nuevo agente.', 'info');
        });
    }
    
    const btnSaveAgent = document.getElementById('btnSaveAgent');
    if (btnSaveAgent) {
        btnSaveAgent.addEventListener('click', saveAgent);
    }
    
    const btnDeleteAgent = document.getElementById('btnDeleteAgent');
    if (btnDeleteAgent) {
        btnDeleteAgent.addEventListener('click', deleteAgent);
    }
    
    const btnResetConversation = document.getElementById('btnResetConversation');
    if (btnResetConversation) {
        btnResetConversation.addEventListener('click', resetAgentConversation);
    }
    
    // Buscar agente al hacer clic en la lista
    const agentsList = document.getElementById('agentsList');
    if (agentsList) {
        // El evento onclick ya está en el HTML inline
    }
    
    // Búsqueda de agentes
    const agentSearch = document.getElementById('agentSearch');
    if (agentSearch) {
        agentSearch.addEventListener('input', (event) => {
            const query = event.target.value.toLowerCase();
            const container = document.getElementById('agentsList');
            if (!container) return;
            
            const agents = state.agents.filter(agent => 
                agent.nombre.toLowerCase().includes(query) || 
                agent.agent_id.toLowerCase().includes(query)
            );
            
            if (!agents.length) {
                container.className = 'card-list empty-state';
                container.innerHTML = 'No se encontraron agentes que coincidan con la búsqueda.';
                return;
            }
            
            container.className = 'card-list';
            container.innerHTML = agents.map(agent => {
                const isSelected = agent.agent_id === state.selectedAgentId;
                return `
                    <article class="entity-card ${isSelected ? 'selected' : ''}" style="cursor: pointer;" 
                             onclick="selectAgentFromTab('${escapeHtml(agent.agent_id)}', '${escapeHtml(agent.nombre)}')">
                        <div class="entity-card-header">
                            <div class="entity-card-title-group">
                                <h4 style="margin: 0;">${escapeHtml(agent.nombre)}</h4>
                                <span class="badge" style="font-size: 0.75em;">${escapeHtml(agent.agent_id)}</span>
                            </div>
                        </div>
                        <div class="form-grid two-cols compact-grid" style="margin-top: 0.5em;">
                            <div class="field">
                                <label>Conversation ID</label>
                                <div class="mono" style="font-size: 0.85em;">${escapeHtml(agent.conversation_id || 'Ninguna')}</div>
                            </div>
                            <div class="field">
                                <label>Último uso</label>
                                <div style="font-size: 0.85em;">${escapeHtml(agent.last_used_at || 'Nunca')}</div>
                            </div>
                        </div>
                    </article>
                `;
            }).join('');
        });
    }
    
    // Al seleccionar un agente de la lista, llenar el formulario
    window.selectAgentFromTab = function(agentId, name) {
        state.selectedAgentId = agentId;
        state.selectedAgentName = name;
        
        const agent = state.agents.find(a => a.agent_id === agentId);
        if (agent) {
            const agentName = document.getElementById('agentName');
            const agentIdInput = document.getElementById('agentId');
            const agentConversationId = document.getElementById('agentConversationId');
            
            if (agentName) agentName.value = agent.nombre || '';
            if (agentIdInput) agentIdInput.value = agent.agent_id || '';
            if (agentConversationId) agentConversationId.value = agent.conversation_id || '';
            
            // Habilitar/deshabilitar botones
            const btnSave = document.getElementById('btnSaveAgent');
            const btnDelete = document.getElementById('btnDeleteAgent');
            const btnReset = document.getElementById('btnResetConversation');
            
            if (btnSave) btnSave.disabled = false;
            if (btnDelete) btnDelete.disabled = false;
            if (btnReset) btnReset.disabled = false;
        }
        
        renderAgentSelectors();
        setStatus(`Agente seleccionado: ${name}`, 'success');
    };
    
    // Binding para catálogos
    const catalogSelect = document.getElementById('catalogSelect');
    if (catalogSelect) {
        catalogSelect.addEventListener('change', (event) => {
            state.selectedCatalog = event.target.value;
            renderCatalogStoryElements();
        });
    }
    
    const storyElementTypeFilter = document.getElementById('storyElementTypeFilter');
    if (storyElementTypeFilter) {
        storyElementTypeFilter.addEventListener('change', (event) => {
            state.selectedStoryElementType = event.target.value;
            renderCatalogStoryElements();
        });
    }
    
    const btnGenerateCatalog = document.getElementById('btnGenerateCatalog');
    if (btnGenerateCatalog) {
        btnGenerateCatalog.addEventListener('click', generateAdaptedCatalog);
    }
    
    const btnHighlightRecommended = document.getElementById('btnHighlightRecommended');
    if (btnHighlightRecommended) {
        btnHighlightRecommended.addEventListener('click', highlightRecommendedElements);
    }
}

function bindGlobalActions() {
    const btnLoadProject = document.getElementById('btnLoadProject');
    if (btnLoadProject) {
      btnLoadProject.addEventListener('click', async () => {
        try {
          await loadProject();
        } catch (error) {
          setStatus(`Error cargando proyecto: ${error.message}`, 'error');
        }
      });
    } else {
      console.warn('[bindGlobalActions] missing element: #btnLoadProject');
    }

    const btnSaveProject = document.getElementById('btnSaveProject');
    if (btnSaveProject) {
      btnSaveProject.addEventListener('click', async () => {
        try {
          await saveProject();
        } catch (error) {
          setStatus(`Error guardando proyecto: ${error.message}`, 'error');
        }
      });
    } else {
      console.warn('[bindGlobalActions] missing element: #btnSaveProject');
    }

    const btnExportJson = document.getElementById('btnExportJson');
    if (btnExportJson) {
      btnExportJson.addEventListener('click', copyRawJson);
    } else {
      console.warn('[bindGlobalActions] missing element: #btnExportJson');
    }

    const workflowBtns = document.querySelectorAll('.workflow-btn');
    workflowBtns.forEach((button) => {
      if (!button) return;
      button.addEventListener('click', () => {
        const section = button.dataset.aiSection;
        const action = button.dataset.aiAction;
        
        // Para acciones específicas de catálogo, usar funciones dedicadas
        if (section === 'plots' && action === 'generate_from_catalog') {
          generatePlotFromCatalog();
          return;
        }
        if (section === 'plots' && action === 'suggest_catalog_elements') {
          suggestCatalogElements();
          return;
        }
        if (section === 'plots' && action === 'validate_with_catalog') {
          validatePlotWithCatalog();
          return;
        }
        
        // Para el resto, usar runAi normal
        runAi(section, action);
      });
    });

    const rawJsonView = document.getElementById('rawJsonView');
    if (rawJsonView) {
      rawJsonView.addEventListener('change', () => {
        const parsed = safeJsonParse(rawJsonView.value, null);
        if (!parsed) {
          setStatus('El JSON no es valido.', 'error');
          return;
        }
        state.project = ensureProjectShape(parsed);
        renderAll();
        setStatus('JSON aplicado al estado del proyecto.', 'success');
      });
    } else {
      console.warn('[bindGlobalActions] missing element: #rawJsonView');
    }

    const btnAddNarrative = document.getElementById('btnAddNarrative');
    if (btnAddNarrative) {
      btnAddNarrative.addEventListener('click', addNarrative);
    } else {
      console.warn('[bindGlobalActions] missing element: #btnAddNarrative');
    }

    const btnAddCharacter = document.getElementById('btnAddCharacter');
    if (btnAddCharacter) {
      btnAddCharacter.addEventListener('click', addCharacter);
    } else {
      console.warn('[bindGlobalActions] missing element: #btnAddCharacter');
    }

    const btnAddPlot = document.getElementById('btnAddPlot');
    if (btnAddPlot) {
      btnAddPlot.addEventListener('click', addPlot);
    } else {
      console.warn('[bindGlobalActions] missing element: #btnAddPlot');
    }

    ["projectMetaTitle","projectMetaFormat","projectMetaSummary","projectMetaWorldContext",
     "projectMetaAllowedGenres","projectMetaMaxRating","projectMetaBlockedTags","projectMetaNotes"
    ].forEach(id => {
        const el = document.getElementById(id);
        if (el) {
            const evt = el.tagName === "SELECT" ? "change" : "input";
            el.addEventListener(evt, () => { syncProjectMetaFromForm(); renderProjectMeta(); renderRawJson(); });
        } else {
            console.warn(`[bindGlobalActions] missing element: #${id}`);
        }
    });

    // Narrative editor bindings live in bindNarrativeForm()
    // to avoid duplicate listeners and duplicate renders.

    ["charName","charProjectRoleType","charSummary","charDescription",
     "charDramaticFunctions","charCoreDrives","charTraits","charToneFit",
     "charAllowedPeriods","charPreferredSettings","charExtendedNotes"
    ].forEach(id => {
        const el = document.getElementById(id);
        if (el) {
            const evt = el.tagName === "SELECT" ? "change" : "input";
            el.addEventListener(evt, () => { syncActiveCharacterFromForm(); renderCharacterList(); renderProjectMeta(); renderRawJson(); });
        } else {
            console.warn(`[bindGlobalActions] missing element: #${id}`);
        }
    });

    ["plotTitle","plotStatus","plotSummary","plotNarrativeLink","plotProtagonistId",
     "plotAntagonistIds","plotSupportingIds","plotThemeIds","plotEventIds",
     "plotSettingIds","plotFinaleId","plotExtendedNotes"
    ].forEach(id => {
        const el = document.getElementById(id);
        if (el) {
            const evt = el.tagName === "SELECT" ? "change" : "input";
            el.addEventListener(evt, () => { syncActivePlotFromForm(); renderPlotList(); renderProjectMeta(); renderRawJson(); });
        } else {
            console.warn(`[bindGlobalActions] missing element: #${id}`);
        }
    });

    const btnAiProjectGenerate = document.getElementById('btnAiProjectGenerate');
    if (btnAiProjectGenerate) {
      btnAiProjectGenerate.addEventListener('click', () => runAi('project', 'generate'));
    } else {
      console.warn('[bindGlobalActions] missing element: #btnAiProjectGenerate');
    }
}

async function bootstrap() {
  try {
    await loadProject();
  } catch (err) {
    console.error('[bootstrap] loadProject failed', err);
  }

  try {
    await loadAgents();
  } catch (err) {
    console.error('[bootstrap] loadAgents failed', err);
  }

  try {
    await loadGenericCatalog();
  } catch (err) {
    console.error('[bootstrap] loadGenericCatalog failed', err);
  }

  try {
    bindTabs();
  } catch (err) {
    console.error('[bootstrap] bindTabs failed', err);
  }

  try {
    bindGlobalActions();
  } catch (err) {
    console.error('[bootstrap] bindGlobalActions failed', err);
  }

  try {
    bindNarrativeForm();
  } catch (err) {
    console.error('[bootstrap] bindNarrativeForm failed', err);
  }

  try {
    bindAgents();
  } catch (err) {
    console.error('[bootstrap] bindAgents failed', err);
  }

  try {
    switchTab('project');
  } catch (err) {
    console.error('[bootstrap] switchTab failed', err);
  }

  try {
    renderAll();
  } catch (err) {
    console.error('[bootstrap] renderAll failed', err);
  }
}

window.addEventListener("DOMContentLoaded", bootstrap);

window.__CADIZ_UI_BUILD__ = '2026-07-12-bootstrap-fix-v3';
console.log('[Cadiz12 UI] build', window.__CADIZ_UI_BUILD__);
