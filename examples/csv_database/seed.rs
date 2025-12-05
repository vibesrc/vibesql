//! Sample data seeding for the CSV database.

use crate::database::CsvDatabase;

/// Seed the database with sample relational data.
///
/// Creates the following tables with relationships:
/// - departments: id, name, budget, manager_id (FK to employees)
/// - employees: id, name, email, department_id (FK to departments), manager_id (FK to employees), salary, hire_date
/// - projects: id, name, department_id (FK to departments), budget, start_date, end_date
/// - project_assignments: employee_id (FK), project_id (FK), role, hours_per_week
/// - skills: id, name, category
/// - employee_skills: employee_id (FK), skill_id (FK), proficiency
pub fn seed_database(db: &mut CsvDatabase) -> Result<(), String> {
    // Clear existing data by dropping and recreating
    println!("Seeding database with sample data...\n");

    // Skills (reference table)
    db.execute("CREATE TABLE skills (id, name, category)")?;
    db.execute("INSERT INTO skills VALUES (1, 'Rust', 'Programming')")?;
    db.execute("INSERT INTO skills VALUES (2, 'Python', 'Programming')")?;
    db.execute("INSERT INTO skills VALUES (3, 'SQL', 'Database')")?;
    db.execute("INSERT INTO skills VALUES (4, 'PostgreSQL', 'Database')")?;
    db.execute("INSERT INTO skills VALUES (5, 'Leadership', 'Soft Skills')")?;
    db.execute("INSERT INTO skills VALUES (6, 'Project Management', 'Soft Skills')")?;
    db.execute("INSERT INTO skills VALUES (7, 'React', 'Programming')")?;
    db.execute("INSERT INTO skills VALUES (8, 'TypeScript', 'Programming')")?;
    db.execute("INSERT INTO skills VALUES (9, 'AWS', 'Infrastructure')")?;
    db.execute("INSERT INTO skills VALUES (10, 'Docker', 'Infrastructure')")?;
    println!("  Created: skills (10 rows)");

    // Departments
    db.execute("CREATE TABLE departments (id, name, budget, location)")?;
    db.execute("INSERT INTO departments VALUES (1, 'Engineering', 2000000, 'Building A')")?;
    db.execute("INSERT INTO departments VALUES (2, 'Product', 800000, 'Building A')")?;
    db.execute("INSERT INTO departments VALUES (3, 'Marketing', 500000, 'Building B')")?;
    db.execute("INSERT INTO departments VALUES (4, 'Sales', 750000, 'Building B')")?;
    db.execute("INSERT INTO departments VALUES (5, 'HR', 300000, 'Building C')")?;
    println!("  Created: departments (5 rows)");

    // Employees (with self-referencing manager_id)
    db.execute(
        "CREATE TABLE employees (id, name, email, department_id, manager_id, salary, hire_date)",
    )?;
    // Engineering
    db.execute("INSERT INTO employees VALUES (1, 'Alice Chen', 'alice@example.com', 1, NULL, 185000, '2019-03-15')")?;
    db.execute("INSERT INTO employees VALUES (2, 'Bob Smith', 'bob@example.com', 1, 1, 145000, '2020-06-01')")?;
    db.execute("INSERT INTO employees VALUES (3, 'Charlie Brown', 'charlie@example.com', 1, 1, 135000, '2020-09-15')")?;
    db.execute("INSERT INTO employees VALUES (4, 'Diana Ross', 'diana@example.com', 1, 2, 125000, '2021-01-10')")?;
    db.execute("INSERT INTO employees VALUES (5, 'Eve Wilson', 'eve@example.com', 1, 2, 120000, '2021-04-01')")?;
    db.execute("INSERT INTO employees VALUES (6, 'Frank Miller', 'frank@example.com', 1, 3, 115000, '2022-02-15')")?;
    // Product
    db.execute("INSERT INTO employees VALUES (7, 'Grace Lee', 'grace@example.com', 2, NULL, 165000, '2019-06-01')")?;
    db.execute("INSERT INTO employees VALUES (8, 'Henry Taylor', 'henry@example.com', 2, 7, 130000, '2020-11-15')")?;
    db.execute("INSERT INTO employees VALUES (9, 'Ivy Johnson', 'ivy@example.com', 2, 7, 125000, '2021-03-01')")?;
    // Marketing
    db.execute("INSERT INTO employees VALUES (10, 'Jack Davis', 'jack@example.com', 3, NULL, 140000, '2019-09-01')")?;
    db.execute("INSERT INTO employees VALUES (11, 'Kate White', 'kate@example.com', 3, 10, 95000, '2021-07-15')")?;
    db.execute("INSERT INTO employees VALUES (12, 'Leo Garcia', 'leo@example.com', 3, 10, 85000, '2022-01-10')")?;
    // Sales
    db.execute("INSERT INTO employees VALUES (13, 'Mary Clark', 'mary@example.com', 4, NULL, 155000, '2018-11-01')")?;
    db.execute("INSERT INTO employees VALUES (14, 'Nick Adams', 'nick@example.com', 4, 13, 105000, '2020-08-15')")?;
    db.execute("INSERT INTO employees VALUES (15, 'Olivia King', 'olivia@example.com', 4, 13, 98000, '2021-05-01')")?;
    // HR
    db.execute("INSERT INTO employees VALUES (16, 'Paul Wright', 'paul@example.com', 5, NULL, 130000, '2019-01-15')")?;
    db.execute("INSERT INTO employees VALUES (17, 'Quinn Hall', 'quinn@example.com', 5, 16, 75000, '2022-03-01')")?;
    println!("  Created: employees (17 rows)");

    // Projects
    db.execute(
        "CREATE TABLE projects (id, name, department_id, budget, start_date, end_date, status)",
    )?;
    db.execute("INSERT INTO projects VALUES (1, 'Platform Rewrite', 1, 500000, '2023-01-01', '2024-06-30', 'active')")?;
    db.execute("INSERT INTO projects VALUES (2, 'Mobile App v2', 1, 300000, '2023-06-01', '2024-03-31', 'active')")?;
    db.execute("INSERT INTO projects VALUES (3, 'Data Pipeline', 1, 200000, '2023-03-15', '2023-12-31', 'completed')")?;
    db.execute("INSERT INTO projects VALUES (4, 'Customer Portal', 2, 150000, '2023-04-01', '2024-02-28', 'active')")?;
    db.execute("INSERT INTO projects VALUES (5, 'Brand Refresh', 3, 100000, '2023-07-01', '2023-12-15', 'completed')")?;
    db.execute("INSERT INTO projects VALUES (6, 'Sales CRM Integration', 4, 80000, '2023-09-01', '2024-04-30', 'active')")?;
    db.execute("INSERT INTO projects VALUES (7, 'Employee Onboarding System', 5, 50000, '2023-08-01', '2024-01-31', 'active')")?;
    println!("  Created: projects (7 rows)");

    // Project Assignments (junction table)
    db.execute(
        "CREATE TABLE project_assignments (id, employee_id, project_id, role, hours_per_week)",
    )?;
    // Platform Rewrite team
    db.execute("INSERT INTO project_assignments VALUES (1, 1, 1, 'Tech Lead', 30)")?;
    db.execute("INSERT INTO project_assignments VALUES (2, 2, 1, 'Senior Engineer', 40)")?;
    db.execute("INSERT INTO project_assignments VALUES (3, 4, 1, 'Engineer', 40)")?;
    db.execute("INSERT INTO project_assignments VALUES (4, 5, 1, 'Engineer', 35)")?;
    // Mobile App team
    db.execute("INSERT INTO project_assignments VALUES (5, 3, 2, 'Tech Lead', 35)")?;
    db.execute("INSERT INTO project_assignments VALUES (6, 6, 2, 'Engineer', 40)")?;
    db.execute("INSERT INTO project_assignments VALUES (7, 8, 2, 'Product Manager', 20)")?;
    // Data Pipeline
    db.execute("INSERT INTO project_assignments VALUES (8, 2, 3, 'Tech Lead', 10)")?;
    db.execute("INSERT INTO project_assignments VALUES (9, 4, 3, 'Engineer', 25)")?;
    // Customer Portal
    db.execute("INSERT INTO project_assignments VALUES (10, 7, 4, 'Product Lead', 30)")?;
    db.execute("INSERT INTO project_assignments VALUES (11, 9, 4, 'Product Manager', 40)")?;
    db.execute("INSERT INTO project_assignments VALUES (12, 3, 4, 'Engineer', 15)")?;
    // Brand Refresh
    db.execute("INSERT INTO project_assignments VALUES (13, 10, 5, 'Lead', 35)")?;
    db.execute("INSERT INTO project_assignments VALUES (14, 11, 5, 'Designer', 40)")?;
    db.execute("INSERT INTO project_assignments VALUES (15, 12, 5, 'Coordinator', 30)")?;
    // Sales CRM
    db.execute("INSERT INTO project_assignments VALUES (16, 13, 6, 'Lead', 25)")?;
    db.execute("INSERT INTO project_assignments VALUES (17, 14, 6, 'Analyst', 40)")?;
    db.execute("INSERT INTO project_assignments VALUES (18, 15, 6, 'Analyst', 35)")?;
    // Onboarding System
    db.execute("INSERT INTO project_assignments VALUES (19, 16, 7, 'Lead', 30)")?;
    db.execute("INSERT INTO project_assignments VALUES (20, 17, 7, 'Coordinator', 40)")?;
    println!("  Created: project_assignments (20 rows)");

    // Employee Skills (junction table with proficiency)
    db.execute("CREATE TABLE employee_skills (id, employee_id, skill_id, proficiency)")?;
    // Alice - Rust, Python, SQL, Leadership
    db.execute("INSERT INTO employee_skills VALUES (1, 1, 1, 'expert')")?;
    db.execute("INSERT INTO employee_skills VALUES (2, 1, 2, 'advanced')")?;
    db.execute("INSERT INTO employee_skills VALUES (3, 1, 3, 'expert')")?;
    db.execute("INSERT INTO employee_skills VALUES (4, 1, 5, 'advanced')")?;
    // Bob - Rust, Python, PostgreSQL, Docker
    db.execute("INSERT INTO employee_skills VALUES (5, 2, 1, 'advanced')")?;
    db.execute("INSERT INTO employee_skills VALUES (6, 2, 2, 'expert')")?;
    db.execute("INSERT INTO employee_skills VALUES (7, 2, 4, 'advanced')")?;
    db.execute("INSERT INTO employee_skills VALUES (8, 2, 10, 'intermediate')")?;
    // Charlie - React, TypeScript, AWS
    db.execute("INSERT INTO employee_skills VALUES (9, 3, 7, 'expert')")?;
    db.execute("INSERT INTO employee_skills VALUES (10, 3, 8, 'expert')")?;
    db.execute("INSERT INTO employee_skills VALUES (11, 3, 9, 'advanced')")?;
    // Diana - Python, SQL, Docker
    db.execute("INSERT INTO employee_skills VALUES (12, 4, 2, 'advanced')")?;
    db.execute("INSERT INTO employee_skills VALUES (13, 4, 3, 'intermediate')")?;
    db.execute("INSERT INTO employee_skills VALUES (14, 4, 10, 'advanced')")?;
    // Eve - Rust, SQL
    db.execute("INSERT INTO employee_skills VALUES (15, 5, 1, 'intermediate')")?;
    db.execute("INSERT INTO employee_skills VALUES (16, 5, 3, 'advanced')")?;
    // Frank - React, TypeScript
    db.execute("INSERT INTO employee_skills VALUES (17, 6, 7, 'intermediate')")?;
    db.execute("INSERT INTO employee_skills VALUES (18, 6, 8, 'intermediate')")?;
    // Grace - Leadership, Project Management
    db.execute("INSERT INTO employee_skills VALUES (19, 7, 5, 'expert')")?;
    db.execute("INSERT INTO employee_skills VALUES (20, 7, 6, 'expert')")?;
    // Add some more variety
    db.execute("INSERT INTO employee_skills VALUES (21, 8, 6, 'advanced')")?;
    db.execute("INSERT INTO employee_skills VALUES (22, 9, 6, 'intermediate')")?;
    db.execute("INSERT INTO employee_skills VALUES (23, 10, 5, 'advanced')")?;
    db.execute("INSERT INTO employee_skills VALUES (24, 13, 5, 'expert')")?;
    db.execute("INSERT INTO employee_skills VALUES (25, 16, 5, 'advanced')")?;
    db.execute("INSERT INTO employee_skills VALUES (26, 16, 6, 'advanced')")?;
    println!("  Created: employee_skills (26 rows)");

    println!("\nDatabase seeded successfully!");
    println!("\nSample queries to try:");
    println!("  -- List employees with their departments");
    println!(
        "  SELECT e.name, d.name FROM employees e JOIN departments d ON e.department_id = d.id"
    );
    println!();
    println!("  -- Find employees and their managers");
    println!("  SELECT e.name, m.name AS manager FROM employees e LEFT JOIN employees m ON e.manager_id = m.id");
    println!();
    println!("  -- Count employees per department");
    println!("  SELECT d.name, COUNT(*) FROM employees e JOIN departments d ON e.department_id = d.id GROUP BY d.name");
    println!();
    println!("  -- List project team members");
    println!("  SELECT p.name, e.name, pa.role FROM project_assignments pa JOIN projects p ON pa.project_id = p.id JOIN employees e ON pa.employee_id = e.id");
    println!();
    println!("  -- Find employee skills");
    println!("  SELECT e.name, s.name, es.proficiency FROM employee_skills es JOIN employees e ON es.employee_id = e.id JOIN skills s ON es.skill_id = s.id");

    Ok(())
}

/// Clear all tables from the database.
pub fn clear_database(db: &mut CsvDatabase, data_dir: &std::path::Path) -> Result<(), String> {
    // Remove all CSV files
    if data_dir.exists() {
        for entry in std::fs::read_dir(data_dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            if path.extension().map(|e| e == "csv").unwrap_or(false) {
                std::fs::remove_file(&path).map_err(|e| e.to_string())?;
            }
        }
    }

    // Recreate the database to clear in-memory state
    *db = CsvDatabase::new(data_dir).map_err(|e| e.to_string())?;

    Ok(())
}
