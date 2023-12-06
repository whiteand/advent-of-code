generate:
    cargo run --bin generate
bench-23d02:
    cargo bench --bench y23d02
test-23d02-task-1:
    cargo watch -x 'test --package advent --lib -- y23::y23d02::tests::test_task1 --exact --nocapture'
test-23d02-task-1-actual:
    cargo watch -x 'test --package advent --lib -- y23::y23d02::tests::test_task1_actual --exact --nocapture'
test-23d02-task-2:
    cargo watch -x 'test --package advent --lib -- y23::y23d02::tests::test_task2 --exact --nocapture'
test-23d02-task-2-actual:
    cargo watch -x 'test --package advent --lib -- y23::y23d02::tests::test_task2_actual --exact --nocapture'
test-23d02:
    cargo watch -x 'test --package advent --lib -- y23::y23d02::tests --nocapture'
bench-y23d03:
    cargo bench --bench y23d03
test-y23d03:
    cargo watch -x 'test --package advent --lib -- y23::y23d03::tests --nocapture'
test-y23d03-task-1:
    cargo watch -x 'test --package advent --lib -- y23::y23d03::tests::test_task1 --exact --nocapture'
test-y23d03-task-1-actual:
    cargo watch -x 'test --package advent --lib -- y23::y23d03::tests::test_task1_actual --exact --nocapture'
test-y23d03-task-2:
    cargo watch -x 'test --package advent --lib -- y23::y23d03::tests::test_task2 --exact --nocapture'
test-y23d03-task-2-actual:
    cargo watch -x 'test --package advent --lib -- y23::y23d03::tests::test_task2_actual --exact --nocapture'

bench-y23d04:
    cargo bench --bench y23d04
test-y23d04:
    cargo watch -x 'test --package advent --lib -- y23::y23d04::tests --nocapture'
test-y23d04-task-1:
    cargo watch -x 'test --package advent --lib -- y23::y23d04::tests::test_task1 --exact --nocapture'
test-y23d04-task-1-actual:
    cargo watch -x 'test --package advent --lib -- y23::y23d04::tests::test_task1_actual --exact --nocapture'
test-y23d04-task-2:
    cargo watch -x 'test --package advent --lib -- y23::y23d04::tests::test_task2 --exact --nocapture'
test-y23d04-task-2-actual:
    cargo watch -x 'test --package advent --lib -- y23::y23d04::tests::test_task2_actual --exact --nocapture'

bench-y23d05:
    cargo bench --bench y23d05
test-y23d05:
    cargo watch -x 'test --package advent --lib -- y23::y23d05::tests --nocapture'
test-y23d05-task-1:
    cargo watch -x 'test --package advent --lib -- y23::y23d05::tests::test_task1 --exact --nocapture'
test-y23d05-task-1-actual:
    cargo watch -x 'test --package advent --lib -- y23::y23d05::tests::test_task1_actual --exact --nocapture'
test-y23d05-task-2:
    cargo watch -x 'test --package advent --lib -- y23::y23d05::tests::test_task2 --exact --nocapture'
test-y23d05-task-2-actual:
    cargo watch -x 'test --package advent --lib -- y23::y23d05::tests::test_task2_actual --exact --nocapture'

bench-y23d06:
    cargo bench --bench y23d06
test-y23d06:
    cargo watch -x 'test --package advent --lib -- y23::y23d06::tests --nocapture'
test-y23d06-task-1:
    cargo watch -x 'test --package advent --lib -- y23::y23d06::tests::test_task1 --exact --nocapture'
test-y23d06-task-1-actual:
    cargo watch -x 'test --package advent --lib -- y23::y23d06::tests::test_task1_actual --exact --nocapture'
test-y23d06-task-2:
    cargo watch -x 'test --package advent --lib -- y23::y23d06::tests::test_task2 --exact --nocapture'
test-y23d06-task-2-actual:
    cargo watch -x 'test --package advent --lib -- y23::y23d06::tests::test_task2_actual --exact --nocapture'
