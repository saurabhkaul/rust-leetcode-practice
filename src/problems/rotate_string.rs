
//https://leetcode.com/problems/rotate-string/?envType=daily-question&envId=2024-11-03
impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        if(s.len()!= goal.len()){
            return false
        };
        let super_str = format!("{}{}",s,s);
        if super_str.contains(&goal){
            return true;
        }else{
            false
        }
    }
}