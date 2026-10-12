import { invoke } from '@tauri-apps/api/core';
import '@fontsource/barlow-condensed/latin-600.css';
import '@fontsource/barlow-condensed/latin-700.css';
import './style.css';

type Settings = { a: number; c: number; d: number };
type Quote = { liters: number; cost: number };

const $ = <T extends HTMLElement = HTMLElement>(id: string) => document.getElementById(id) as T;
const remaining = $<HTMLInputElement>('remaining');
const maximum = $<HTMLInputElement>('maximum');
const efficiency = $<HTMLInputElement>('efficiency');
const price = $<HTMLInputElement>('price');
const save = $<HTMLButtonElement>('save');
const details = $<HTMLDetailsElement>('settings');

const format = (value: number) => value.toLocaleString('ko-KR');
const parse = (input: HTMLInputElement) => {
  const text = input.value.trim();
  return /^\d+(\.\d+)?$/.test(text) ? Number(text) : NaN;
};

let settings: Settings | null = null;
let latest = 0; // 가장 최근 계산 요청 번호. 이전 요청의 응답은 버린다.

function say(text: string, isError = false) {
  $('message').textContent = text;
  $('message').classList.toggle('error', isError);
}

function show(quote: Quote | null, b: number) {
  $('cost').textContent = quote ? format(quote.cost) : '—';
  $('currency').textContent = quote ? '원' : '';
  $('liters').textContent = quote ? `${quote.liters.toFixed(2)} L` : '—';
  const max = quote && settings ? settings.a : null;
  $('gauge').classList.toggle('ready', max !== null);
  $('from').textContent = max !== null ? `${format(b)} km` : '—';
  $('to').textContent = max !== null ? `${format(Math.max(0, max - b))} km` : '—';
  $('fill').style.width = max !== null ? `${Math.min(100, (b / max) * 100)}%` : '0%';
}

async function refresh() {
  const id = ++latest;
  const b = parse(remaining);
  const entered = remaining.value.trim() !== '';
  let quote: Quote | null = null;
  let problem = '';
  if (entered && !Number.isFinite(b)) {
    problem = '남은 거리는 0 이상의 숫자로 입력하세요.';
  } else if (entered && !settings) {
    problem = '계산 기준 설정을 먼저 저장하세요.';
  } else if (entered) {
    try {
      quote = await invoke<Quote>('calculate', { settings, b });
    } catch (error) {
      problem = `계산하지 못했습니다. (${error})`;
    }
    if (id !== latest) return;
  }
  $('error').textContent = problem;
  show(quote, b);
}

function apply(next: Settings | null) {
  settings = next;
  $('basis').textContent = next
    ? `${format(next.a)} km · ${format(next.c)} km/L · ${format(next.d)} 원/L`
    : '저장된 설정 없음';
  void refresh();
}

remaining.addEventListener('input', () => void refresh());

$('settings-form').addEventListener('submit', async (event) => {
  event.preventDefault();
  if (save.disabled) return;
  const fields = [[maximum, '최대 거리'], [efficiency, '연비'], [price, '주유단가']] as const;
  for (const [input, label] of fields) {
    const value = parse(input);
    if (!Number.isFinite(value) || value <= 0) {
      say(`${label}는 0보다 큰 숫자로 입력하세요.`, true);
      input.focus();
      return;
    }
  }
  const next = { a: parse(maximum), c: parse(efficiency), d: parse(price) };
  save.disabled = true;
  try {
    await invoke('save_settings', { settings: next });
    apply(next);
    say('설정을 저장했습니다.');
  } catch (error) {
    say(`설정을 저장하지 못했습니다. 입력한 값은 그대로 두었습니다. (${error})`, true);
  } finally {
    save.disabled = false;
  }
});

async function init() {
  try {
    const saved = await invoke<Settings | null>('load_settings');
    if (saved) {
      maximum.value = String(saved.a);
      efficiency.value = String(saved.c);
      price.value = String(saved.d);
    }
    details.open = !saved;
    apply(saved);
  } catch (error) {
    details.open = true;
    say(`저장된 설정을 불러오지 못했습니다. 값을 입력해 다시 저장하세요. (${error})`, true);
    apply(null);
  } finally {
    save.disabled = false;
  }
}

void init();
