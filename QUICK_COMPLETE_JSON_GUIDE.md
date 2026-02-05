# 🚀 Quick Guide: Complete JSON Export

## วิธีใช้งาน JSON Viewer ใหม่

### 1. โหลดไฟล์ .flop

```bash
npm run tauri dev
```

1. คลิก **"Load"** บนแถบ navbar
2. เลือกไฟล์ `.flop` หรือ `.bin`
3. รอให้โหลดเสร็จ

→ JSON Viewer จะเปิดอัตโนมัติแสดง **Metadata**

---

### 2. Toggle ระหว่าง Metadata และ Complete

ใน JSON Viewer คุณจะเห็น 2 ปุ่ม:

#### 📋 **Metadata Mode** (ค่าเริ่มต้น)
- ขนาด: ~500 bytes
- แสดง: memo, board, pot, stack
- ❌ ไม่มี: strategy, equity, EV

#### 🎯 **Complete Mode** (ต้อง solve ก่อน!)
- ขนาด: ~150 MB
- แสดง: **ทุกอย่าง** รวมถึง:
  - ✅ Ranges (OOP & IP)
  - ✅ Strategy (กลยุทธ์การเล่น)
  - ✅ Equity (โอกาสชนะ)
  - ✅ EV (มูลค่าที่คาดหวัง)
  - ✅ EQR (Equity Realization)
  - ✅ Weights & Normalizer

---

### 3. Export JSON

#### วิธีที่ 1: Export จาก JSON Viewer
1. เลือก Mode ที่ต้องการ (Metadata หรือ Complete)
2. คลิก **"Export"**
3. เลือกตำแหน่งและชื่อไฟล์
4. ✅ เสร็จ!

#### วิธีที่ 2: Copy to Clipboard
1. คลิก **"Copy JSON"**
2. Paste ไปใช้ที่อื่น
3. ✅ เสร็จ!

---

## 🎮 ตัวอย่างการใช้งาน

### Scenario 1: ดูข้อมูลพื้นฐาน (เร็ว)

```
1. Load file → JSON Viewer เปิด
2. ดู Metadata mode (แสดงทันที)
3. Export หรือ Copy
```

**เวลา**: 1-2 วินาที
**ขนาด**: ~500 bytes

---

### Scenario 2: ดูข้อมูลครบถ้วน (ช้า แต่ครบ!)

```
1. Load file → JSON Viewer เปิด
2. รัน Solver (ถ้ายังไม่ได้ solve)
3. คลิก "Complete" mode
4. รอ 10-20 วินาที (กำลังโหลดข้อมูล)
5. Export หรือ Copy
```

**เวลา**: 10-20 วินาที (ครั้งแรก)
**ขนาด**: ~150 MB

---

## 📊 JSON Structure

### Metadata Mode
```json
{
  "version": 1,
  "memo": "BTN vs BB - Kh Qh 7c",
  "is_solved": true,
  "storage_mode": "flop",
  "board": [46, 42, 20],
  "starting_pot": 60,
  "effective_stack": 940,
  "exported_at": "2026-02-05T10:00:00Z"
}
```

### Complete Mode
```json
{
  "version": 1,
  "memo": "BTN vs BB - Kh Qh 7c",
  "exported_at": "2026-02-05T10:00:00Z",

  "game_config": {
    "is_solved": true,
    "storage_mode": "flop",
    "board": [46, 42, 20],
    "board_readable": "Kh Qh 7c",
    "starting_pot": 60,
    "effective_stack": 940
  },

  "ranges": {
    "oop": "66+,A8s+,AJo+,K9s+",
    "ip": "QQ-22,AQs-A2s,ATo+"
  },

  "private_cards": {
    "oop": [258, 257, 256, ...],
    "ip": [770, 769, 768, ...]
  },

  "results": {
    "current_player": "oop",
    "num_actions": 4,

    "weights": { "oop": [...], "ip": [...] },
    "equity": { "oop": [...], "ip": [...] },
    "ev": { "oop": [...], "ip": [...] },
    "eqr": { "oop": [...], "ip": [...] },
    "strategy": [...],
    "action_ev": [...]
  }
}
```

---

## 🔑 Card Encoding

```
card_number = rank * 4 + suit

Ranks: 0=2, 1=3, 2=4, ..., 12=A
Suits: 0=♣, 1=♦, 2=♥, 3=♠

Examples:
- Kh = 11 * 4 + 2 = 46
- Qh = 10 * 4 + 2 = 42
- 7c = 5 * 4 + 0 = 20
```

### Decode in JavaScript
```javascript
const ranks = ['2','3','4','5','6','7','8','9','T','J','Q','K','A'];
const suits = ['c','d','h','s'];

function decodeCard(n) {
  return ranks[Math.floor(n/4)] + suits[n%4];
}

decodeCard(46); // "Kh"
```

---

## ⚠️ Important Notes

### Metadata Mode
- ✅ เร็ว (1-2 วินาที)
- ✅ ขนาดเล็ก (~500 bytes)
- ❌ ไม่มีข้อมูล strategy/equity

### Complete Mode
- ✅ ครบถ้วน (มีทุกอย่าง!)
- ⚠️ ช้า (10-20 วินาที load ครั้งแรก)
- ⚠️ ขนาดใหญ่ (~150 MB)
- ⚠️ ต้อง solve ก่อน

---

## 🐛 Troubleshooting

### ❌ ปุ่ม "Complete" disabled
**สาเหตุ**: ยังไม่ได้รัน solver
**แก้ไข**: รัน solver ให้เสร็จก่อน

### ❌ "Loading complete data..." นาน
**เป็นปกติ**: Complete mode ใช้เวลา 10-20 วินาที
**คำแนะนำ**: รอให้เสร็จ หรือกลับไป Metadata mode

### ❌ Out of Memory
**สาเหตุ**: RAM ไม่พอ
**แก้ไข**:
- ปิดโปรแกรมอื่นๆ
- ใช้ Metadata mode แทน
- ใช้เกมที่เล็กกว่า

---

## 📚 ไฟล์ที่เกี่ยวข้อง

- [`COMPLETE_JSON_EXPORT.md`](./COMPLETE_JSON_EXPORT.md) - เอกสารฉบับเต็ม
- [`src/utils/exportComplete.ts`](./src/utils/exportComplete.ts) - โค้ด export
- [`src/components/JsonViewer.vue`](./src/components/JsonViewer.vue) - UI component

---

## 💡 Use Cases

### 1. Documentation
Export metadata เพื่อบันทึกการตั้งค่าเกม

### 2. Analysis
Export complete เพื่อวิเคราะห์ strategy ด้วย Python/R

### 3. Sharing
แชร์ JSON configuration กับทีม

### 4. Backup
บันทึกข้อมูลเพื่อเก็บไว้อ้างอิง

---

**สรุป**: ใช้ **Metadata** สำหรับดูข้อมูลพื้นฐาน ใช้ **Complete** สำหรับวิเคราะห์ละเอียด! 🎯

**เวอร์ชัน**: 1.0.0
**วันที่**: 2026-02-05
