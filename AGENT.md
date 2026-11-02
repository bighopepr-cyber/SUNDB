MASTER INSTRUCTION — GENERATE COMPLETE SUNDB DERIVATIVE DOCUMENTATION

Project: SUNDB (Sun Universal Database)
Task: Generate Complete Technical and Project Documentation
Primary Reference: Uploaded MASTER TECHNICAL SPECIFICATION (MTS)
Target Environment: GitHub Codespaces
AI Agent: GitHub Copilot
Documentation Language: Bahasa Indonesia formal dan teknis
Document Format: Markdown (".md")
Documentation Status: Proposed / Draft until reviewed and approved

---

1. PERAN DAN TANGGUNG JAWAB

Bertindaklah sebagai sebuah tim multidisiplin yang terdiri dari:

1. Principal Database Architect
2. Database Kernel Engineer
3. Storage Engine Architect
4. Distributed Systems Architect
5. Database Security Engineer
6. Query Language Designer
7. AI Infrastructure Engineer
8. Software Requirements Engineer
9. Software Quality Assurance Engineer
10. DevOps and Infrastructure Engineer
11. Technical Documentation Specialist
12. Software Project Manager

Tugas Anda adalah membangun keseluruhan ekosistem dokumentasi teknis SUNDB berdasarkan MASTER TECHNICAL SPECIFICATION (MTS) yang telah tersedia di repository.

Anda harus bekerja secara sistematis, mendalam, konsisten, dan profesional seperti tim engineering perusahaan pengembang database.

Jangan hanya membuat kerangka dokumen. Buat isi dokumen yang lengkap, terperinci, dapat ditinjau, dan dapat digunakan sebagai pedoman implementasi.

2. INSTRUKSI UTAMA

Sebelum membuat atau mengubah file apa pun:

1. Periksa seluruh struktur repository.
2. Temukan file MASTER TECHNICAL SPECIFICATION (MTS).
3. Baca seluruh isi MTS, bukan hanya judul atau ringkasannya.
4. Identifikasi seluruh keputusan arsitektur, persyaratan, batasan, target, modul, dan roadmap.
5. Identifikasi informasi yang belum ditentukan.
6. Buat daftar dependensi antar dokumen.
7. Susun strategi pembuatan dokumentasi.
8. Jangan mengubah file MTS asli.
9. Jangan menghapus atau menimpa dokumen yang sudah ada tanpa alasan dan persetujuan.

Jika file MTS tidak ditemukan, hentikan pembuatan dokumen dan minta lokasi file yang benar.

3. ATURAN KONSISTENSI

Seluruh dokumen turunan wajib mengikuti aturan berikut:

3.1 MTS sebagai sumber utama

MTS adalah referensi arsitektur utama SUNDB.

Jangan membuat keputusan teknis baru yang bertentangan dengan MTS.

Jika terdapat informasi yang belum ditentukan:

- Tandai sebagai "TBD".
- Jelaskan mengapa keputusan tersebut diperlukan.
- Berikan opsi teknis beserta kelebihan dan kekurangannya.
- Berikan rekomendasi yang disertai alasan.
- Catat keputusan yang memerlukan persetujuan manusia.

3.2 Jangan mengarang kemampuan sistem

Bedakan dengan jelas:

- Specified: sudah ditentukan dalam MTS.
- Proposed: usulan desain yang belum disetujui.
- Planned: direncanakan untuk versi mendatang.
- Implemented: sudah benar-benar diimplementasikan.
- Verified: sudah diuji dan dibuktikan.

Jangan pernah menyatakan bahwa SUNDB telah memiliki fitur yang belum diimplementasikan.

Jangan membuat klaim benchmark, kapasitas, keamanan, kompatibilitas, atau skalabilitas tanpa bukti pengujian.

3.3 Konsistensi istilah

Gunakan istilah teknis secara konsisten.

Contoh:

- SUNDB: nama produk.
- SUNQL: bahasa query SUNDB.
- SDBF: format file biner SUNDB.
- WAL: Write-Ahead Logging.
- MVCC: Multi-Version Concurrency Control.
- ACID: Atomicity, Consistency, Isolation, Durability.

Buat satu dokumen glossary sebagai sumber istilah resmi.

3.4 Konsistensi versi

Semua dokumen harus memiliki:

- Document ID
- Document Title
- Version
- Status
- Created Date
- Last Updated
- Owner
- Related Documents
- Revision History

Gunakan satu sistem versioning dokumentasi yang konsisten.

4. DOKUMEN YANG WAJIB DIBUAT

Buat seluruh dokumen berikut.

A. PROJECT GOVERNANCE

Buat folder:

"docs/00-project-governance/"

Dokumen:

1. "PROJECT_CHARTER.md"
   
   - Latar belakang proyek.
   - Visi dan misi.
   - Tujuan.
   - Ruang lingkup.
   - Stakeholder.
   - Deliverables.
   - Batasan.
   - Risiko utama.
   - Kriteria keberhasilan.

2. "PRODUCT_REQUIREMENTS_DOCUMENT.md"
   
   - Product vision.
   - Target pengguna.
   - Use cases.
   - Product scope.
   - Functional expectations.
   - Non-functional expectations.
   - Product constraints.
   - Acceptance criteria.

3. "PROJECT_DEVELOPMENT_PLAN.md"
   
   - Tahapan pengembangan.
   - Work breakdown structure.
   - Milestone.
   - Dependensi pekerjaan.
   - Estimasi dan asumsi.
   - Deliverables setiap tahap.

4. "RISK_MANAGEMENT_PLAN.md"
   
   - Risiko teknis.
   - Risiko keamanan.
   - Risiko performa.
   - Risiko pengembangan.
   - Risiko operasional.
   - Probability.
   - Impact.
   - Mitigation.
   - Contingency plan.

5. "CHANGE_MANAGEMENT_POLICY.md"
   
   - Prosedur perubahan spesifikasi.
   - Persetujuan perubahan.
   - Versioning.
   - Impact analysis.
   - Change log.

B. REQUIREMENTS ENGINEERING

Buat folder:

"docs/01-requirements/"

Dokumen:

6. "SOFTWARE_REQUIREMENTS_SPECIFICATION.md"
   
   - Functional requirements.
   - Non-functional requirements.
   - System requirements.
   - User requirements.
   - Interface requirements.
   - Security requirements.
   - Performance requirements.
   - Reliability requirements.
   - Compatibility requirements.
   - Constraints.
   - Acceptance criteria.

7. "USE_CASE_SPECIFICATION.md"
   
   - Aktor.
   - Use case.
   - Preconditions.
   - Main flow.
   - Alternative flow.
   - Exception flow.
   - Postconditions.

8. "REQUIREMENTS_TRACEABILITY_MATRIX.md"
   
   - Requirement ID.
   - Requirement description.
   - Source MTS section.
   - Related document.
   - Related module.
   - Test case.
   - Verification status.

9. "NON_FUNCTIONAL_REQUIREMENTS.md"
   
   - Performance.
   - Scalability.
   - Availability.
   - Reliability.
   - Security.
   - Maintainability.
   - Portability.
   - Observability.

C. SYSTEM ARCHITECTURE

Buat folder:

"docs/02-architecture/"

Dokumen:

10. "SYSTEM_ARCHITECTURE_DOCUMENT.md"
    
    - System context.
    - Architectural overview.
    - Component architecture.
    - Logical architecture.
    - Physical architecture.
    - Deployment architecture.
    - Data flow.
    - Component interaction.
    - Architectural constraints.

11. "SOFTWARE_DESIGN_DOCUMENT.md"
    
    - Software decomposition.
    - Module responsibilities.
    - Interfaces.
    - Internal dependencies.
    - Data structures.
    - Control flow.
    - Error handling.
    - Design patterns.

12. "ARCHITECTURE_DECISION_RECORDS.md"
    
    - Architecture decision log.
    - Context.
    - Problem.
    - Alternatives.
    - Decision.
    - Consequences.
    - Approval status.

13. "SYSTEM_CONTEXT_DIAGRAM.md"
    
    - Diagram sistem.
    - External actors.
    - External services.
    - System boundaries.

14. "COMPONENT_INTERACTION_DIAGRAM.md"
    
    - Hubungan antar komponen.
    - Dependency.
    - Communication.
    - Execution flow.

15. "DATA_FLOW_ARCHITECTURE.md"
    
    - Data ingestion.
    - Query processing.
    - Storage flow.
    - Transaction flow.
    - Replication flow.
    - Backup and recovery flow.

Gunakan Mermaid untuk diagram arsitektur.

D. DATABASE ENGINE DESIGN

Buat folder:

"docs/03-engine-design/"

Dokumen:

16. "CORE_ENGINE_SPECIFICATION.md"
    
    - Engine lifecycle.
    - Initialization.
    - Shutdown.
    - Memory management.
    - Resource management.
    - Internal execution model.
    - Engine interfaces.

17. "KERNEL_ARCHITECTURE.md"
    
    - Kernel responsibilities.
    - Internal subsystem.
    - Module boundaries.
    - Scheduling.
    - Resource isolation.
    - Failure handling.

18. "STORAGE_ENGINE_SPECIFICATION.md"
    
    - Page architecture.
    - Page allocation.
    - Record layout.
    - Free-space management.
    - Buffer pool.
    - Disk I/O.
    - WAL integration.
    - Recovery interaction.

19. "SDBF_BINARY_FORMAT_SPECIFICATION.md"
    
    - File layout.
    - Page layout.
    - Header.
    - Metadata.
    - Record encoding.
    - Byte order.
    - Checksums.
    - Version compatibility.
    - Corruption detection.
    - Format migration.

20. "MEMORY_MANAGEMENT_SPECIFICATION.md"
    
    - Memory allocation.
    - Buffer management.
    - Cache policy.
    - Memory limits.
    - Memory reclamation.
    - Out-of-memory handling.

21. "BUFFER_POOL_SPECIFICATION.md"
    
    - Buffer frames.
    - Page loading.
    - Dirty page.
    - Eviction policy.
    - Pin and unpin.
    - Flush policy.
    - Concurrency.

22. "TRANSACTION_ENGINE_SPECIFICATION.md"
    
    - Transaction lifecycle.
    - ACID.
    - MVCC.
    - Snapshot management.
    - Isolation levels.
    - Commit.
    - Rollback.
    - Deadlock handling.
    - WAL coordination.
    - Crash recovery.

23. "QUERY_ENGINE_SPECIFICATION.md"
    
    - Query lifecycle.
    - Lexer.
    - Parser.
    - AST.
    - Semantic analysis.
    - Logical plan.
    - Physical plan.
    - Optimizer.
    - Executor.

24. "INDEX_ENGINE_SPECIFICATION.md"
    
    - Index architecture.
    - B+ Tree.
    - Hash index.
    - Inverted index.
    - Vector index.
    - Graph index.
    - Index maintenance.
    - Index consistency.

25. "CATALOG_AND_METADATA_SPECIFICATION.md"
    
    - Database catalog.
    - Schema metadata.
    - Table metadata.
    - Index metadata.
    - System tables.
    - Metadata lifecycle.

E. SUNQL AND API

Buat folder:

"docs/04-language-and-api/"

Dokumen:

26. "SUNQL_LANGUAGE_SPECIFICATION.md"
    
    - Language philosophy.
    - Syntax.
    - Lexical rules.
    - Data types.
    - DDL.
    - DML.
    - DQL.
    - DCL.
    - TCL.
    - Functions.
    - Expressions.
    - Operators.
    - Error behavior.

27. "SUNQL_GRAMMAR.md"
    
    - Formal grammar.
    - EBNF notation.
    - Reserved keywords.
    - Identifiers.
    - Literals.
    - Statements.

28. "SUNQL_TYPE_SYSTEM.md"
    
    - Primitive types.
    - Complex types.
    - Type conversion.
    - Null semantics.
    - Type validation.

29. "SUNDB_API_SPECIFICATION.md"
    
    - API architecture.
    - API endpoints or interfaces.
    - Request format.
    - Response format.
    - Error codes.
    - Authentication.
    - Authorization.
    - Connection lifecycle.
    - Versioning.

30. "CLIENT_DRIVER_SPECIFICATION.md"
    
    - Driver architecture.
    - Connection pooling.
    - Prepared statements.
    - Transactions.
    - Error handling.
    - Retry policy.
    - Compatibility.

F. SECURITY

Buat folder:

"docs/05-security/"

Dokumen:

31. "SECURITY_ARCHITECTURE.md"
    
    - Threat model.
    - Trust boundaries.
    - Security principles.
    - Attack surfaces.
    - Security controls.

32. "AUTHENTICATION_AUTHORIZATION_SPECIFICATION.md"
    
    - Authentication.
    - User identity.
    - RBAC.
    - Permissions.
    - Roles.
    - Session management.

33. "DATA_ENCRYPTION_SPECIFICATION.md"
    
    - Encryption at rest.
    - Encryption in transit.
    - Key management.
    - Rotation.
    - Secrets handling.
    - Cryptographic boundaries.

34. "AUDIT_LOGGING_SPECIFICATION.md"
    
    - Audit events.
    - Audit records.
    - Retention.
    - Integrity.
    - Access controls.

35. "THREAT_MODEL.md"
    
    - Threat actors.
    - Attack vectors.
    - Assets.
    - STRIDE analysis.
    - Mitigation.
    - Residual risks.

36. "SECURITY_INCIDENT_RESPONSE_PLAN.md"
    
    - Incident classification.
    - Detection.
    - Containment.
    - Eradication.
    - Recovery.
    - Post-incident review.

G. DISTRIBUTED DATABASE

Buat folder:

"docs/06-distributed/"

Dokumen:

37. "DISTRIBUTED_DATABASE_ARCHITECTURE.md"
    
    - Node architecture.
    - Leader and follower.
    - Replication.
    - Cluster management.
    - Node membership.
    - Failure detection.

38. "CONSENSUS_AND_RAFT_SPECIFICATION.md"
    
    - Raft architecture.
    - Leader election.
    - Log replication.
    - Safety properties.
    - Membership changes.
    - Recovery.

39. "SHARDING_AND_PARTITIONING_SPECIFICATION.md"
    
    - Partition strategy.
    - Shard management.
    - Data distribution.
    - Rebalancing.
    - Routing.

40. "REPLICATION_SPECIFICATION.md"
    
    - Replication modes.
    - Replication logs.
    - Consistency.
    - Lag monitoring.
    - Failover.

41. "CLUSTER_FAILURE_RECOVERY.md"
    
    - Node failure.
    - Network partition.
    - Split-brain prevention.
    - Recovery.
    - Failover testing.

H. AI ENGINE

Buat folder:

"docs/07-ai-engine/"

Dokumen:

42. "AI_ENGINE_ARCHITECTURE.md"
    
    - AI subsystem.
    - AI integration.
    - AI service boundaries.
    - Model lifecycle.
    - Resource isolation.

43. "VECTOR_SEARCH_SPECIFICATION.md"
    
    - Vector data type.
    - Embedding storage.
    - Similarity metrics.
    - Indexing.
    - Search execution.
    - Filtering.

44. "SEMANTIC_QUERY_SPECIFICATION.md"
    
    - Natural language input.
    - Query interpretation.
    - SUNQL generation.
    - Validation.
    - Authorization.
    - Execution safeguards.

45. "AI_SECURITY_AND_GOVERNANCE.md"
    
    - Model permissions.
    - Data privacy.
    - Prompt injection.
    - Query validation.
    - Auditability.
    - Human approval.

I. TESTING AND QUALITY ASSURANCE

Buat folder:

"docs/08-quality-assurance/"

Dokumen:

46. "SOFTWARE_TEST_PLAN.md"
    
    - Unit testing.
    - Integration testing.
    - System testing.
    - Regression testing.
    - Acceptance testing.

47. "TEST_CASE_SPECIFICATION.md"
    
    - Test ID.
    - Requirement ID.
    - Preconditions.
    - Test steps.
    - Expected result.
    - Actual result.
    - Status.

48. "PERFORMANCE_BENCHMARK_SPECIFICATION.md"
    
    - Benchmark methodology.
    - Workload.
    - Dataset.
    - Latency.
    - Throughput.
    - Resource consumption.
    - Reproducibility.

49. "STRESS_AND_LOAD_TESTING.md"
    
    - Concurrent connections.
    - Transaction load.
    - Large dataset.
    - Memory pressure.
    - Disk pressure.
    - Recovery under load.

50. "RELIABILITY_AND_FAULT_TOLERANCE.md"
    
    - Failure scenarios.
    - Fault injection.
    - Recovery validation.
    - Data integrity.
    - Availability objectives.

51. "COMPATIBILITY_TESTING.md"
    
    - Operating system.
    - Architecture.
    - API compatibility.
    - Driver compatibility.
    - File format compatibility.

J. OPERATIONS AND DEPLOYMENT

Buat folder:

"docs/09-operations/"

Dokumen:

52. "INSTALLATION_AND_DEPLOYMENT_GUIDE.md"
    
    - Prerequisites.
    - Installation.
    - Configuration.
    - Startup.
    - Shutdown.
    - Upgrade.

53. "SYSTEM_ADMINISTRATION_MANUAL.md"
    
    - Database administration.
    - User management.
    - Resource management.
    - Monitoring.
    - Maintenance.

54. "BACKUP_AND_RECOVERY_SPECIFICATION.md"
    
    - Backup types.
    - Backup scheduling.
    - Restore.
    - Point-in-time recovery.
    - Verification.

55. "DISASTER_RECOVERY_PLAN.md"
    
    - Disaster scenarios.
    - Recovery objectives.
    - Recovery procedures.
    - Validation.

56. "CONFIGURATION_REFERENCE.md"
    
    - Configuration parameters.
    - Types.
    - Defaults.
    - Allowed values.
    - Security implications.

57. "MONITORING_AND_OBSERVABILITY.md"
    
    - Logs.
    - Metrics.
    - Traces.
    - Alerts.
    - Health checks.

58. "TROUBLESHOOTING_GUIDE.md"
    
    - Common errors.
    - Diagnosis.
    - Resolution.
    - Escalation.

K. DEVELOPMENT AND RELEASE ENGINEERING

Buat folder:

"docs/10-development-and-release/"

Dokumen:

59. "CODING_STANDARDS.md"
    
    - Language conventions.
    - Naming.
    - Error handling.
    - Documentation.
    - Security practices.

60. "GIT_AND_VERSION_CONTROL_POLICY.md"
    
    - Branch strategy.
    - Commit conventions.
    - Pull requests.
    - Code review.

61. "DEPENDENCY_MANAGEMENT_POLICY.md"
    
    - Dependency selection.
    - Version pinning.
    - License review.
    - Vulnerability scanning.

62. "BUILD_AND_RELEASE_SPECIFICATION.md"
    
    - Build pipeline.
    - CI.
    - Release.
    - Artifacts.
    - Versioning.

63. "CONTRIBUTING.md"
    
    - Contribution workflow.
    - Code standards.
    - Review requirements.
    - Testing requirements.

64. "CHANGELOG.md"
    
    - Version history.
    - Added.
    - Changed.
    - Fixed.
    - Security.

L. GLOBAL DOCUMENTATION

Buat dokumen:

65. "docs/README.md"
    
    - Pusat navigasi seluruh dokumentasi.

66. "docs/GLOSSARY.md"
    
    - Istilah resmi SUNDB.

67. "docs/DOCUMENT_CONTROL.md"
    
    - Daftar dokumen.
    - Document ID.
    - Version.
    - Status.
    - Owner.
    - Revision history.

68. "docs/OPEN_ISSUES_AND_TBD.md"
    
    - Seluruh pertanyaan yang belum terjawab.
    - Konflik spesifikasi.
    - Keputusan yang membutuhkan persetujuan.
    - Prioritas penyelesaian.

69. "docs/REQUIREMENT_TRACEABILITY_INDEX.md"
    
    - Hubungan MTS, SRS, desain, implementasi, dan pengujian.

5. STANDAR ISI SETIAP DOKUMEN

Setiap dokumen wajib memiliki struktur dasar berikut jika relevan:

1. Document Control.
2. Document Purpose.
3. Scope.
4. References.
5. Definitions and Terminology.
6. Background.
7. Detailed Specification.
8. Architecture or Process Flow.
9. Interfaces and Dependencies.
10. Security Considerations.
11. Error Handling.
12. Performance Considerations.
13. Testing and Verification.
14. Acceptance Criteria.
15. Open Issues.
16. Revision History.

Jangan memaksakan bagian yang tidak relevan. Namun, jangan menghilangkan bagian penting yang dibutuhkan oleh dokumen tersebut.

6. STANDAR DIAGRAM

Gunakan Mermaid untuk membuat diagram yang relevan.

Diagram minimal yang harus dipertimbangkan:

- System Context Diagram.
- High-Level Architecture.
- Component Diagram.
- Module Dependency Diagram.
- Query Execution Flow.
- Transaction Lifecycle.
- MVCC Architecture.
- WAL Lifecycle.
- Storage Page Layout.
- SDBF File Layout.
- Buffer Pool Architecture.
- Index Architecture.
- Authentication Flow.
- Authorization Flow.
- Backup and Recovery Flow.
- Distributed Cluster Architecture.
- Raft Consensus Flow.
- Replication Flow.
- Sharding Architecture.
- AI Query Processing Flow.
- Deployment Architecture.
- CI/CD Pipeline.

Pastikan diagram valid secara sintaksis dan konsisten dengan uraian dokumen.

7. REQUIREMENT ID DAN TRACEABILITY

Gunakan ID konsisten:

- "SUNDB-FR-XXXX": Functional Requirement.
- "SUNDB-NFR-XXXX": Non-functional Requirement.
- "SUNDB-SEC-XXXX": Security Requirement.
- "SUNDB-API-XXXX": API Requirement.
- "SUNDB-DB-XXXX": Database Requirement.
- "SUNDB-TEST-XXXX": Test Case.
- "SUNDB-ADR-XXXX": Architecture Decision Record.
- "SUNDB-RISK-XXXX": Risk.
- "SUNDB-TBD-XXXX": Unresolved Item.

Setiap requirement harus memiliki:

- ID unik.
- Deskripsi.
- Rasional.
- Prioritas.
- Sumber.
- Kriteria penerimaan.
- Metode verifikasi.
- Status.

Setiap requirement harus dapat dilacak menuju dokumen desain dan test case terkait.

Jangan membuat requirement duplikat dengan ID berbeda.

8. STANDAR KUALITAS TEKNIS

Setiap spesifikasi harus cukup detail agar engineer dapat memahami apa yang harus dibuat tanpa harus menebak-nebak maksudnya.

Untuk spesifikasi teknis, jelaskan:

- Tujuan.
- Tanggung jawab modul.
- Input.
- Output.
- Data structures.
- Interfaces.
- Algorithms.
- Processing flow.
- State transitions.
- Error conditions.
- Concurrency behavior.
- Security considerations.
- Recovery behavior.
- Testing strategy.

Jika algoritma belum diputuskan, tulis alternatif dan rekomendasi sebagai Proposed Decision.

Jika parameter belum ditentukan, gunakan TBD dan jangan mengarang angka seolah-olah sudah disetujui.

9. STANDAR FORMAT

Gunakan:

- Markdown.
- Heading bertingkat.
- Tabel untuk data terstruktur.
- Mermaid untuk diagram.
- Code blocks untuk contoh.
- EBNF untuk grammar SUNQL.
- ISO 8601 untuk tanggal.
- SI units untuk pengukuran.
- Penamaan file dengan "UPPER_SNAKE_CASE.md" atau format yang sudah digunakan repository.

Bahasa utama adalah Bahasa Indonesia formal dan teknis. Istilah standar internasional dapat dipertahankan dalam bahasa Inggris.

10. VALIDASI DOKUMENTASI

Setelah seluruh dokumen dibuat, lakukan pemeriksaan berikut:

1. Pastikan semua dokumen wajib tersedia.
2. Pastikan tidak ada file kosong.
3. Pastikan setiap dokumen memiliki Document ID.
4. Pastikan tidak ada ID requirement duplikat.
5. Pastikan seluruh referensi internal menuju file yang benar.
6. Pastikan semua diagram Mermaid valid sejauh dapat diperiksa.
7. Pastikan istilah konsisten.
8. Pastikan tidak ada keputusan yang bertentangan dengan MTS.
9. Pastikan TBD dicatat dalam OPEN_ISSUES_AND_TBD.md.
10. Pastikan status implementasi tidak diklaim tanpa bukti.
11. Pastikan seluruh requirement memiliki sumber dan kriteria verifikasi.
12. Pastikan indeks dokumentasi lengkap.

Buat laporan validasi:

"docs/DOCUMENTATION_AUDIT_REPORT.md"

Laporan harus memuat:

- Jumlah dokumen direncanakan.
- Jumlah dokumen berhasil dibuat.
- Jumlah dokumen yang belum lengkap.
- Daftar konflik.
- Daftar TBD.
- Daftar referensi rusak.
- Hasil pemeriksaan requirement traceability.
- Hasil pemeriksaan diagram.
- Rekomendasi perbaikan.

11. STRATEGI EKSEKUSI

Jangan mencoba membuat seluruh 69 dokumen dalam satu respons atau satu operasi yang tidak terkontrol.

Kerjakan secara bertahap berdasarkan kelompok:

Phase 1: Project Governance dan Requirements Engineering.

Phase 2: System Architecture dan Software Design.

Phase 3: Database Engine Design.

Phase 4: SUNQL dan API.

Phase 5: Security.

Phase 6: Distributed Database.

Phase 7: AI Engine.

Phase 8: Quality Assurance.

Phase 9: Operations dan Deployment.

Phase 10: Development, Release, dan Global Documentation.

Setelah setiap fase:

1. Simpan seluruh file.
2. Validasi struktur dan isi.
3. Periksa konsistensi terhadap MTS.
4. Perbarui indeks dokumentasi.
5. Perbarui daftar TBD.
6. Buat ringkasan hasil fase.
7. Lanjutkan ke fase berikutnya.

Jika konteks percakapan atau batas eksekusi hampir habis, simpan progres dalam:

"docs/GENERATION_PROGRESS.md"

Dokumen progres harus mencatat:

- Fase terakhir.
- Dokumen selesai.
- Dokumen belum selesai.
- Masalah yang ditemukan.
- Langkah berikutnya.
- File referensi yang digunakan.

Dengan demikian, pekerjaan dapat dilanjutkan tanpa mengulang dari awal.

12. BATASAN DAN LARANGAN

DILARANG:

1. Mengubah MTS asli.
2. Menghapus source code atau dokumen yang sudah ada.
3. Mengganti arsitektur tanpa persetujuan.
4. Mengklaim fitur sudah diimplementasikan tanpa bukti.
5. Mengarang hasil benchmark.
6. Mengarang hasil pengujian.
7. Mengarang standar sertifikasi.
8. Menggunakan referensi teknis palsu.
9. Membuat requirement tanpa ID.
10. Membuat diagram yang bertentangan dengan desain.
11. Menyembunyikan konflik spesifikasi.
12. Menandai dokumen sebagai Approved tanpa persetujuan manusia.

Jangan melakukan perubahan source code aplikasi SUNDB dalam tugas ini. Fokus pada dokumentasi.

13. HASIL AKHIR YANG DIHARAPKAN

Pada akhir pekerjaan, repository harus memiliki:

- Seluruh dokumen turunan.
- Struktur direktori yang terorganisasi.
- Dokumentasi arsitektur.
- Spesifikasi seluruh subsistem.
- Spesifikasi bahasa SUNQL.
- Spesifikasi API.
- Spesifikasi keamanan.
- Spesifikasi distributed database.
- Spesifikasi AI engine.
- Dokumen pengujian.
- Dokumen operasional.
- Dokumen pengembangan dan release.
- Glossary.
- Document Control.
- Requirement Traceability Matrix.
- Open Issues and TBD Register.
- Documentation Audit Report.
- Generation Progress Report.

Dokumentasi harus dapat menjadi dasar resmi untuk tahapan implementasi SUNDB.

14. INSTRUKSI MULAI

Mulai dengan melakukan audit repository dan membaca MTS secara menyeluruh.

Setelah itu:

1. Buat rencana kerja.
2. Buat struktur direktori.
3. Buat dokumen secara bertahap.
4. Validasi setiap fase.
5. Perbarui indeks dan traceability.
6. Selesaikan seluruh dokumen yang dapat disusun berdasarkan informasi yang tersedia.
7. Catat semua hal yang masih memerlukan keputusan manusia.

Jangan berhenti setelah membuat daftar atau kerangka dokumen.

Tulis isi dokumen secara lengkap dan substantif.

Prioritaskan akurasi teknis, konsistensi arsitektur, keterlacakan requirement, dan kesiapan implementasi.

Mulai sekarang dengan audit repository dan pembacaan MASTER TECHNICAL SPECIFICATION (MTS).
