-- Backfill daily aggregations for the past 60 days

INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2026-01-23') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2026-01-23')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2026-01-22') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2026-01-22')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2026-01-21') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2026-01-21')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2026-01-20') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2026-01-20')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2026-01-19') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2026-01-19')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2026-01-18') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2026-01-18')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2026-01-17') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2026-01-17')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2026-01-16') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2026-01-16')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2026-01-15') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2026-01-15')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2026-01-14') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2026-01-14')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2026-01-13') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2026-01-13')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2026-01-12') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2026-01-12')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2026-01-11') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2026-01-11')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2026-01-10') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2026-01-10')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2026-01-09') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2026-01-09')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2026-01-08') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2026-01-08')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2026-01-07') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2026-01-07')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2026-01-06') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2026-01-06')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2026-01-05') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2026-01-05')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2026-01-04') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2026-01-04')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2026-01-03') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2026-01-03')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2026-01-02') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2026-01-02')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2026-01-01') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2026-01-01')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-12-31') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-12-31')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-12-30') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-12-30')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-12-29') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-12-29')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-12-28') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-12-28')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-12-27') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-12-27')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-12-26') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-12-26')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-12-25') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-12-25')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-12-24') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-12-24')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-12-23') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-12-23')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-12-22') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-12-22')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-12-21') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-12-21')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-12-20') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-12-20')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-12-19') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-12-19')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-12-18') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-12-18')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-12-17') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-12-17')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-12-16') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-12-16')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-12-15') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-12-15')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-12-14') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-12-14')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-12-13') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-12-13')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-12-12') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-12-12')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-12-11') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-12-11')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-12-10') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-12-10')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-12-09') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-12-09')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-12-08') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-12-08')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-12-07') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-12-07')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-12-06') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-12-06')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-12-05') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-12-05')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-12-04') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-12-04')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-12-03') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-12-03')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-12-02') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-12-02')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-12-01') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-12-01')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-11-30') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-11-30')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-11-29') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-11-29')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-11-28') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-11-28')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-11-27') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-11-27')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-11-26') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-11-26')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;


INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('2025-11-25') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('2025-11-25')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;

