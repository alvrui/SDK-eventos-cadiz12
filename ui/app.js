const state = {
    project: createDefaultProject(),
    activeTab: 'project',
    activeNarrativeId: null,
    activeCharacterId: null,
    activePlotId: null,
    storyFilterType: "All",
    activity: [],
    agents: []
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
    const container = $("#agentOverview");

    if (!state.agents.length) {
        container.className = "agent-overview empty-state";
        container.innerHTML = "No se pudieron cargar agentes.";
        return;
    }

    container.className = "agent-overview";
    container.innerHTML = state.agents.map((agent) => `
        <div class="agent-pill">
            <div class="agent-pill-name">${escapeHtml(agent.nombre || agent.name || "-")}</div>
            <div class="agent-pill-id">${escapeHtml(agent.agent_id || agent.id || "")}</div>
        </div>
    `).join("");
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
  renderStoryElements();
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

function getFilteredStoryElements() {
    if (state.storyFilterType === "All") return state.project.storyelements;
    return state.project.storyelements.filter((item) => item.type === state.storyFilterType);
}

function renderStoryElements() {
    const container = $("#storyElementsList");
    const filtered = getFilteredStoryElements();

    if (!filtered.length) {
        container.className = "card-list empty-state";
        container.innerHTML = "No hay story elements para este filtro.";
        populateStoryElementSelects();
        renderProjectMeta();
        renderRawJson();
        return;
    }

    container.className = "card-list";
    container.innerHTML = "";

    filtered.forEach((item) => {
        const template = $("#storyElementCardTemplate");
        const fragment = template.content.cloneNode(true);
        const card = fragment.querySelector(".story-card");

        card.dataset.storyId = item.id;

        const idInput = fragment.querySelector(".story-id-input");
        const labelInput = fragment.querySelector(".story-label-input");
        const typeInput = fragment.querySelector(".story-type-input");
        const toneInput = fragment.querySelector(".story-tone-input");
        const descInput = fragment.querySelector(".story-description-input");
        const selectedInput = fragment.querySelector(".story-selected-input");
        const removeBtn = fragment.querySelector(".story-remove-btn");
        const toEventBtn = fragment.querySelector(".story-to-event-btn");

        idInput.value = item.id || "";
        labelInput.value = item.label || "";
        typeInput.value = item.type || "Theme";
        toneInput.value = item.tone || "";
        descInput.value = item.description || "";
        selectedInput.checked = !!item.selected;

        idInput.addEventListener("input", syncStoryElementsFromDom);
        labelInput.addEventListener("input", syncStoryElementsFromDom);
        typeInput.addEventListener("change", syncStoryElementsFromDom);
        toneInput.addEventListener("input", syncStoryElementsFromDom);
        descInput.addEventListener("input", syncStoryElementsFromDom);
        selectedInput.addEventListener("change", syncStoryElementsFromDom);

        removeBtn.addEventListener("click", () => {
            state.project.storyelements = state.project.storyelements.filter((story) => story.id !== item.id);
            renderStoryElements();
            populateStoryElementSelects();
            renderProjectMeta();
            renderRawJson();
            setStatus("Story element eliminado.", "success");
        });

        toEventBtn.addEventListener("click", () => {
            if (!item.id) return;
            $("#eventStoryElementLink").value = item.id;
            switchTab("events");
            setStatus(`Story element ${item.label || item.id} seleccionado para evento.`, "success");
        });

        container.appendChild(fragment);
    });

    populateStoryElementSelects();
    renderProjectMeta();
    renderRawJson();
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
    populateStoryElementSelects();
}

function addStoryElement(type = "Theme") {
    syncProjectFromForms();
    state.project.storyelements.push(createEmptyStoryElement(type));
    renderStoryElements();
    setStatus("Story element añadido.", "success");
}

function renderEvents() {
    const container = $("#eventsList");
    if (!state.project.events.length) {
        container.className = "card-list empty-state";
        container.innerHTML = "No hay eventos todavía.";
        populateStoryElementSelects();
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

    populateStoryElementSelects();
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

function populateStoryElementSelects() {
    const mainSelect = $("#eventStoryElementLink");
    if (mainSelect) {
        populateStorySelect(mainSelect, mainSelect.value);
    }

    $all(".event-story-element-id-input").forEach((select) => {
        const selected = select.value;
        populateStorySelect(select, selected);
    });
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

    setStatus(`Ejecutando IA: ${section} / ${action}...`, "info");

    try {
        let payloadProject = clone(state.project);

        if (section === 'event' && action === 'generate_from_story_element') {
            const selectedStoryId = document.getElementById('eventStoryElementLink')?.value;
            if (selectedStoryId) payloadProject.selectedStoryElementId = selectedStoryId;
        }

        const response = await apiPost(endpoint, {
            action,
            project: payloadProject
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
        renderStoryElements();
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
